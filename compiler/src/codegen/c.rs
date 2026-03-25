// C codegen backend — AST to .n2c compiled contract
use crate::ast::*;
use super::{CodeGenerator, CodegenError, CompilationMeta, collect_states, clean_pattern};

pub struct CBackend;

impl CodeGenerator for CBackend {
    fn target_name(&self) -> &str { "c" }
    fn file_extension(&self) -> &str { ".n2c" }

    fn generate(&self, ast: &N2File, meta: &CompilationMeta) -> Result<String, CodegenError> {
        let mut out = String::with_capacity(4096);

        emit_header(&mut out, meta);
        emit_prelude(&mut out);

        for block in &ast.blocks {
            match block {
                Block::Contract(ct) => emit_contract(&mut out, ct),
                Block::Rule(r) => emit_rule(&mut out, r),
                Block::Workflow(w) => emit_workflow(&mut out, w),
                // Schema codegen is only supported for Rust target
                _ => {}
            }
        }

        out.push_str("\n#endif /* N2_CONTRACT_H */\n");
        Ok(out)
    }
}

fn emit_header(out: &mut String, meta: &CompilationMeta) {
    out.push_str(&format!(
        "/* N2 Compiled Contract — C target\n\
         \x20* Source: {}\n\
         \x20* Version: {}\n\
         \x20* Compiler: {}\n\
         \x20* Compiled: {}\n\
         \x20*/\n\n",
        meta.source_name, meta.source_version,
        meta.compiler_version, meta.compiled_at
    ));
}

fn emit_prelude(out: &mut String) {
    out.push_str(
        "#ifndef N2_CONTRACT_H\n\
         #define N2_CONTRACT_H\n\n\
         #include <string.h>\n\
         #include <stdio.h>\n\n\
         #define N2_OK 0\n\
         #define N2_ERR_INVALID_TRANSITION -1\n\
         #define N2_ERR_BLACKLISTED -2\n\n"
    );
}

fn emit_contract(out: &mut String, ct: &ContractBlock) {
    if ct.transitions.is_empty() { return; }

    let name_lower = ct.name.to_lowercase();
    let states = collect_states(&ct.transitions);

    // Emit enum
    out.push_str(&format!("/* State machine: {} */\n", ct.name));
    out.push_str(&format!("typedef enum {{\n"));
    for (i, s) in states.iter().enumerate() {
        out.push_str(&format!("    {}_{} = {},\n", name_lower.to_uppercase(), s, i));
    }
    out.push_str(&format!("}} {}_state_t;\n\n", name_lower));

    // Emit transition function
    out.push_str(&format!(
        "static int {name}_transition({name}_state_t *current, const char *event) {{\n",
        name = name_lower
    ));
    for t in &ct.transitions {
        out.push_str(&format!(
            "    if (*current == {}_{} && strcmp(event, \"{}\") == 0) {{\n\
             \x20       *current = {}_{};\n\
             \x20       return N2_OK;\n\
             \x20   }}\n",
            name_lower.to_uppercase(), t.from,
            t.on_event,
            name_lower.to_uppercase(), t.to,
        ));
    }
    out.push_str(
        "    return N2_ERR_INVALID_TRANSITION;\n\
         }\n\n"
    );

    // Emit state name function
    out.push_str(&format!(
        "static const char* {}_state_name({}_state_t s) {{\n\
         \x20   switch (s) {{\n",
        name_lower, name_lower
    ));
    for s in &states {
        out.push_str(&format!(
            "        case {}_{}: return \"{}\";\n",
            name_lower.to_uppercase(), s, s
        ));
    }
    out.push_str(
        "        default: return \"UNKNOWN\";\n\
         \x20   }\n\
         }\n\n"
    );
}

fn emit_rule(out: &mut String, rule: &RuleBlock) {
    if rule.blacklist.is_empty() { return; }

    let name_lower = rule.name.to_lowercase();
    out.push_str(&format!(
        "/* Rule: {} */\n\
         static int {}_check_blacklist(const char *input) {{\n",
        rule.name, name_lower
    ));
    for p in &rule.blacklist {
            let clean = clean_pattern(p);
        out.push_str(&format!(
            "    if (strstr(input, \"{}\") != NULL) return N2_ERR_BLACKLISTED;\n",
            clean
        ));
    }
    out.push_str("    return N2_OK;\n}\n\n");
}

fn emit_workflow(out: &mut String, wf: &WorkflowBlock) {
    let name_lower = wf.name.to_lowercase();
    out.push_str(&format!(
        "/* Workflow: {} */\n\
         static const char* {}_steps[] = {{\n",
        wf.name, name_lower
    ));
    for step in &wf.steps {
        out.push_str(&format!("    \"{}\",\n", step.name));
    }
    out.push_str("};\n");
    out.push_str(&format!(
        "static const int {}_step_count = {};\n\n",
        name_lower, wf.steps.len()
    ));
}
