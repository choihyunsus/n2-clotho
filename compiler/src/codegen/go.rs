// Go codegen backend — AST to .n2go compiled contract
use crate::ast::*;
use super::{CodeGenerator, CodegenError, CompilationMeta, collect_states, clean_pattern};

pub struct GoBackend;

impl CodeGenerator for GoBackend {
    fn target_name(&self) -> &str { "go" }
    fn file_extension(&self) -> &str { ".n2go" }

    fn generate(&self, ast: &N2File, meta: &CompilationMeta) -> Result<String, CodegenError> {
        let mut out = String::with_capacity(4096);

        // Scan AST to determine which imports are needed
        let has_contract = ast.blocks.iter().any(|b| {
            matches!(b, Block::Contract(c) if !c.transitions.is_empty())
        });
        let has_rule_blacklist = ast.blocks.iter().any(|b| {
            matches!(b, Block::Rule(r) if !r.blacklist.is_empty())
        });

        emit_header(&mut out, meta);
        emit_prelude(&mut out, has_contract, has_rule_blacklist);

        for block in &ast.blocks {
            match block {
                Block::Contract(ct) => emit_contract(&mut out, ct),
                Block::Rule(r) => emit_rule(&mut out, r),
                Block::Workflow(w) => emit_workflow(&mut out, w),
                // Schema codegen is only supported for Rust target
                _ => {}
            }
        }

        Ok(out)
    }
}

fn emit_header(out: &mut String, meta: &CompilationMeta) {
    out.push_str(&format!(
        "// N2 Compiled Contract — Go target\n\
         // Source: {}\n\
         // Version: {}\n\
         // Compiler: {}\n\
         // Compiled: {}\n\n",
        meta.source_name, meta.source_version,
        meta.compiler_version, meta.compiled_at
    ));
}

fn emit_prelude(out: &mut String, needs_fmt: bool, needs_strings: bool) {
    out.push_str("package n2contract\n\n");

    // Only emit imports that are actually used to satisfy Go compiler
    if needs_fmt || needs_strings {
        out.push_str("import (\n");
        if needs_fmt {
            out.push_str("\t\"fmt\"\n");
        }
        if needs_strings {
            out.push_str("\t\"strings\"\n");
        }
        out.push_str(")\n\n");
    }

    if needs_fmt {
        out.push_str(
            "// ContractViolation represents an invalid state transition\n\
             type ContractViolation struct {\n\
             \tContract  string\n\
             \tFromState string\n\
             \tEvent     string\n\
             }\n\n\
             func (e *ContractViolation) Error() string {\n\
             \treturn fmt.Sprintf(\"[%s] invalid transition from %s on '%s'\", e.Contract, e.FromState, e.Event)\n\
             }\n\n"
        );
    }
}

fn emit_contract(out: &mut String, ct: &ContractBlock) {
    if ct.transitions.is_empty() { return; }

    let name = &ct.name;
    let states = collect_states(&ct.transitions);

    // Emit state constants
    out.push_str(&format!("// State machine: {}\n", name));
    out.push_str(&format!("type {}State int\n\n", name));
    out.push_str("const (\n");
    for (i, s) in states.iter().enumerate() {
        if i == 0 {
            out.push_str(&format!("\t{}_{} {}State = iota\n", name, s, name));
        } else {
            out.push_str(&format!("\t{}_{}\n", name, s));
        }
    }
    out.push_str(")\n\n");

    // State name map
    out.push_str(&format!("var {}StateNames = map[{}State]string{{\n", name, name));
    for s in &states {
        out.push_str(&format!("\t{name}_{s}: \"{s}\",\n", name = name, s = s));
    }
    out.push_str("}\n\n");

    // Struct
    out.push_str(&format!(
        "type {} struct {{\n\tState {}State\n}}\n\n",
        name, name
    ));

    // Constructor
    out.push_str(&format!(
        "func New{}() *{} {{\n\treturn &{}{{State: {name}_{first}}}\n}}\n\n",
        name, name, name, name = name, first = states[0]
    ));

    // Transition
    out.push_str(&format!(
        "func (c *{}) Transition(event string) error {{\n\tswitch {{\n",
        name
    ));
    for t in &ct.transitions {
        out.push_str(&format!(
            "\tcase c.State == {name}_{from} && event == \"{ev}\":\n\
             \t\tc.State = {name}_{to}\n\
             \t\treturn nil\n",
            name = name, from = t.from, to = t.to, ev = t.on_event
        ));
    }
    out.push_str(&format!(
        "\tdefault:\n\
         \t\treturn &ContractViolation{{Contract: \"{name}\", FromState: {name}StateNames[c.State], Event: event}}\n\
         \t}}\n}}\n\n",
        name = name
    ));
}

fn emit_rule(out: &mut String, rule: &RuleBlock) {
    if rule.blacklist.is_empty() { return; }

    out.push_str(&format!("// Rule: {}\n", rule.name));
    out.push_str(&format!(
        "func {}CheckBlacklist(input string) (string, bool) {{\n\
         \tpatterns := []string{{\n",
        rule.name
    ));
    for p in &rule.blacklist {
        let clean = clean_pattern(p);
        out.push_str(&format!("\t\t\"{}\",\n", clean));
    }
    out.push_str(
        "\t}\n\
         \tfor _, p := range patterns {\n\
         \t\tif strings.Contains(input, p) {\n\
         \t\t\treturn p, true\n\
         \t\t}\n\
         \t}\n\
         \treturn \"\", false\n\
         }\n\n"
    );
}

fn emit_workflow(out: &mut String, wf: &WorkflowBlock) {
    out.push_str(&format!("// Workflow: {}\n", wf.name));
    out.push_str(&format!("var {}Steps = []string{{\n", wf.name));
    for step in &wf.steps {
        out.push_str(&format!("\t\"{}\",\n", step.name));
    }
    out.push_str("}\n\n");
}
