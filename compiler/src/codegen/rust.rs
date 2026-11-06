// Rust codegen backend — AST to .n2rs compiled contract
use crate::ast::*;
use super::{CodeGenerator, CodegenError, CompilationMeta, collect_states};

pub struct RustBackend;

impl CodeGenerator for RustBackend {
    fn target_name(&self) -> &str { "rust" }
    fn file_extension(&self) -> &str { ".n2rs" }

    fn generate(&self, ast: &N2File, meta: &CompilationMeta) -> Result<String, CodegenError> {
        super::patterns::validate(ast, self.target_name())?;
        let mut out = String::with_capacity(4096);

        emit_header(&mut out, meta);
        emit_prelude(&mut out);

        for block in &ast.blocks {
            match block {
                Block::Contract(ct) => emit_contract(&mut out, ct),
                Block::Rule(r) => emit_rule(&mut out, r),
                Block::Workflow(w) => emit_workflow(&mut out, w),
                Block::Schema(s) => emit_schema(&mut out, s),
                _ => {}
            }
        }

        Ok(out)
    }
}

fn emit_header(out: &mut String, meta: &CompilationMeta) {
    out.push_str(&format!(
        "// N2 Compiled Contract — Rust target\n\
         // Source: {}\n\
         // Version: {}\n\
         // Compiler: {}\n\
         // Compiled: {}\n\n",
        super::header_value(&meta.source_name), super::header_value(&meta.source_version),
        super::header_value(&meta.compiler_version), super::header_value(&meta.compiled_at)
    ));
}

fn emit_prelude(out: &mut String) {
    out.push_str(
        "use std::collections::HashMap;\n\n\
         /// Contract violation error\n\
         #[derive(Debug)]\n\
         pub struct ContractViolation {\n\
         \x20   pub contract: String,\n\
         \x20   pub from_state: String,\n\
         \x20   pub event: String,\n\
         \x20   pub message: String,\n\
         }\n\n"
    );
}

fn emit_contract(out: &mut String, ct: &ContractBlock) {
    if ct.transitions.is_empty() { return; }

    let name = &ct.name;

    let states = collect_states(&ct.transitions);
    emit_contract_start(out, name, &states);

    out.push_str(
        "    pub fn current_state(&self) -> &str {\n\
         \x20       match self.state {\n"
    );
    for s in &states {
        out.push_str(&format!("            {}State::{} => \"{}\",\n", name, s, s));
    }
    out.push_str("        }\n    }\n\n");

    out.push_str(
        "    pub fn transition(&mut self, event: &str) -> Result<&str, ContractViolation> {\n\
         \x20       let next = match (&self.state, event) {\n"
    );
    for t in &ct.transitions {
        out.push_str(&format!(
            "            ({}State::{}, \"{}\") => {}State::{},\n",
            name, t.from, t.on_event, name, t.to
        ));
    }
    out.push_str(&format!(
        "            _ => return Err(ContractViolation {{\n\
         \x20               contract: \"{}\".into(),\n\
         \x20               from_state: format!(\"{{:?}}\", self.state),\n\
         \x20               event: event.into(),\n\
         \x20               message: format!(\"invalid transition from {{:?}} on '{{}}'\", self.state, event),\n\
         \x20           }}),\n",
        name
    ));
    out.push_str(
        "        };\n\
         \x20       self.state = next;\n\
         \x20       Ok(self.current_state())\n\
         \x20   }\n"
    );
    out.push_str("}\n\n");
}

fn emit_contract_start(out: &mut String, name: &str, states: &[String]) {
    // Emit state enum
    out.push_str(&format!("/// State machine: {}\n", name));
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    out.push_str(&format!("pub enum {}State {{\n", name));
    for s in states {
        out.push_str(&format!("    {},\n", s));
    }
    out.push_str("}\n\n");

    // Emit contract struct
    out.push_str(&format!(
        "pub struct {} {{\n\
         \x20   state: {}State,\n\
         }}\n\n",
        name, name
    ));

    // Emit impl
    out.push_str(&format!("impl {} {{\n", name));
    out.push_str(&format!(
        "    pub fn new() -> Self {{\n\
         \x20       {} {{ state: {}State::{} }}\n\
         \x20   }}\n\n",
        name, name, states[0]
    ));

}

fn emit_rule(out: &mut String, rule: &RuleBlock) {
    out.push_str(&format!("pub struct {}Rule;\nimpl {}Rule {{\n", rule.name, rule.name));
    out.push_str("    // Requires the regex crate (1.x) in the consuming Cargo.toml.\n    pub fn check_blacklist(input: &str) -> Option<&'static str> {\n");
    for p in &rule.blacklist {
        let pattern = super::patterns::expression(p);
        out.push_str(&format!("        if regex::Regex::new({:?}).expect(\"validated blacklist\").is_match(input) {{ return Some({:?}); }}\n", pattern, p));
    }
    out.push_str("        None\n    }\n}\n\n");
}

fn emit_workflow(out: &mut String, wf: &WorkflowBlock) {
    let name = &wf.name;
    out.push_str(&format!("/// Workflow: {}\n", name));
    out.push_str(&format!("pub struct {}Workflow;\n\n", name));
    out.push_str(&format!("impl {}Workflow {{\n", name));
    out.push_str("    pub const STEPS: &'static [&'static str] = &[\n");
    for step in &wf.steps {
        out.push_str(&format!("        \"{}\",\n", step.name));
    }
    out.push_str("    ];\n");
    out.push_str("}\n\n");
}

fn emit_schema(out: &mut String, schema: &SchemaBlock) {
    for def in &schema.definitions {
        if def.is_enum {
            out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
            out.push_str(&format!("pub enum {} {{\n", def.name));
            for f in &def.fields {
                out.push_str(&format!("    {},\n", f.name));
            }
            out.push_str("}\n\n");
        }
    }
}
