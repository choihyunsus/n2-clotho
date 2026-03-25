// C++ codegen backend — AST to .n2c2 compiled contract
use crate::ast::*;
use super::{CodeGenerator, CodegenError, CompilationMeta, collect_states, clean_pattern};

pub struct CppBackend;

impl CodeGenerator for CppBackend {
    fn target_name(&self) -> &str { "cpp" }
    fn file_extension(&self) -> &str { ".n2c2" }

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

        out.push_str("} // namespace n2\n");
        Ok(out)
    }
}

fn emit_header(out: &mut String, meta: &CompilationMeta) {
    out.push_str(&format!(
        "// N2 Compiled Contract — C++ target\n\
         // Source: {}\n\
         // Version: {}\n\
         // Compiler: {}\n\
         // Compiled: {}\n\n",
        meta.source_name, meta.source_version,
        meta.compiler_version, meta.compiled_at
    ));
}

fn emit_prelude(out: &mut String) {
    out.push_str(
        "#pragma once\n\
         #include <string>\n\
         #include <stdexcept>\n\
         #include <vector>\n\n\
         namespace n2 {\n\n\
         class ContractViolation : public std::runtime_error {\n\
         public:\n\
         \x20   explicit ContractViolation(const std::string& msg)\n\
         \x20       : std::runtime_error(msg) {}\n\
         };\n\n"
    );
}

fn emit_contract(out: &mut String, ct: &ContractBlock) {
    if ct.transitions.is_empty() { return; }

    let name = &ct.name;
    let states = collect_states(&ct.transitions);

    // Emit enum class
    out.push_str(&format!("/// State machine: {}\n", name));
    out.push_str(&format!("enum class {}State {{\n", name));
    for s in &states {
        out.push_str(&format!("    {},\n", s));
    }
    out.push_str("};\n\n");

    // Emit class
    out.push_str(&format!(
        "class {} {{\n\
         \x20   {}State state_ = {}State::{};\n\
         public:\n",
        name, name, name, states[0]
    ));

    // current_state()
    out.push_str("    const char* currentState() const {\n");
    out.push_str("        switch (state_) {\n");
    for s in &states {
        out.push_str(&format!(
            "            case {}State::{}: return \"{}\";\n",
            name, s, s
        ));
    }
    out.push_str("        }\n        return \"UNKNOWN\";\n    }\n\n");

    // transition()
    out.push_str(&format!(
        "    {}State transition(const std::string& event) {{\n\
         \x20       switch (state_) {{\n",
        name
    ));

    // Group transitions by from-state
    let mut from_groups: Vec<(String, Vec<&TransitionStmt>)> = Vec::new();
    for t in &ct.transitions {
        if let Some(g) = from_groups.iter_mut().find(|(f, _)| *f == t.from) {
            g.1.push(t);
        } else {
            from_groups.push((t.from.clone(), vec![t]));
        }
    }

    for (from, transitions) in &from_groups {
        out.push_str(&format!("            case {}State::{}:\n", name, from));
        for t in transitions {
            out.push_str(&format!(
                "                if (event == \"{}\") {{ state_ = {}State::{}; return state_; }}\n",
                t.on_event, name, t.to
            ));
        }
        out.push_str("                break;\n");
    }

    out.push_str("            default: break;\n");
    out.push_str("        }\n");
    out.push_str(&format!(
        "        throw ContractViolation(\"invalid transition from \" + \
         std::string(currentState()) + \" on '\" + event + \"'\");\n\
         \x20   }}\n"
    ));
    out.push_str("};\n\n");
}

fn emit_rule(out: &mut String, rule: &RuleBlock) {
    if rule.blacklist.is_empty() { return; }

    out.push_str(&format!("/// Rule: {}\n", rule.name));
    out.push_str(&format!("class {}Rule {{\npublic:\n", rule.name));
    out.push_str("    static bool checkBlacklist(const std::string& input) {\n");
    out.push_str("        static const std::vector<std::string> patterns = {\n");
    for p in &rule.blacklist {
            let clean = clean_pattern(p);
        out.push_str(&format!("            \"{}\",\n", clean));
    }
    out.push_str("        };\n");
    out.push_str("        for (const auto& p : patterns) {\n");
    out.push_str("            if (input.find(p) != std::string::npos) return true;\n");
    out.push_str("        }\n");
    out.push_str("        return false;\n");
    out.push_str("    }\n};\n\n");
}

fn emit_workflow(out: &mut String, wf: &WorkflowBlock) {
    out.push_str(&format!("/// Workflow: {}\n", wf.name));
    out.push_str(&format!(
        "struct {}Workflow {{\n\
         \x20   static constexpr const char* steps[] = {{\n",
        wf.name
    ));
    for step in &wf.steps {
        out.push_str(&format!("        \"{}\",\n", step.name));
    }
    out.push_str("    };\n};\n\n");
}
