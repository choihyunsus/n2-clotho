use super::*;
use crate::parser::parse_n2;

fn meta() -> CompilationMeta {
    CompilationMeta { source_name: "Probe".into(), source_version: "1".into(),
        target: String::new(), extension: String::new(), compiler_version: "test".into(),
        compiled_at: "2026-10-31".into() }
}

fn rejects_all(body: &str) {
    let ast = parse_n2(&format!("@meta {{ name: \"Probe\" enforce: strict }}\n{body}")).unwrap();
    for (target, _, result) in BackendRegistry::new().compile_all(&ast, &meta()) {
        assert!(result.is_err(), "{target} silently accepted unsupported or invalid behavior: {body}");
    }
}

#[test]
fn contract_conditions_are_never_silently_discarded() {
    rejects_all(r#"@contract Gate { states: Mode precondition { $input.ready == true => "required" }
        transitions { Idle -> Done : on go } }"#);
}

#[test]
fn rule_checks_are_never_silently_discarded() {
    rejects_all(r#"@rule Safe { check { $input.ready == true => "required" } blacklist: [/blocked/] }"#);
}

#[test]
fn ambiguous_transitions_are_rejected_across_targets() {
    rejects_all("@contract Gate { states: Mode transitions { Idle -> Ready : on go Idle -> Done : on go } }");
}

#[test]
fn nonportable_identifiers_are_rejected() {
    rejects_all("@contract bad-name { states: Mode transitions { Idle -> Done : on go } }");
    rejects_all("@contract Gate { states: Mode transitions { class -> Done : on go } }");
}

#[test]
fn executable_workflow_and_recipe_require_the_runtime() {
    rejects_all(r#"@workflow Work { step Run { tool: "file_write" } }"#);
    rejects_all(r#"@recipe Work { step Run { tool: "file_write" } }"#);
}

#[test]
fn compilation_metadata_cannot_escape_generated_comments() {
    let ast = parse_n2(r#"@meta { name: "Probe" enforce: strict } @rule Safe { blacklist: [/blocked/] }"#).unwrap();
    let mut metadata = meta();
    metadata.source_name = "*/\nINJECTED_METADATA\r\n".into();
    for (target, _, result) in BackendRegistry::new().compile_all(&ast, &metadata) {
        let output = result.unwrap();
        assert!(!output.lines().any(|line| line == "INJECTED_METADATA"), "{target} emits metadata as source code");
    }
}

#[test]
fn registry_queries_apply_projection_filter_order_and_limit() {
    let source = r#"@meta { name: "Probe" enforce: strict }
        @rule Alpha { scope: files enforce: strict blacklist: [/one/] }
        @rule Beta { scope: network enforce: warn blacklist: [/one/, /two/] }"#;
    let registry = crate::query::N2Registry::from_file(&parse_n2(source).unwrap());
    let filtered = registry.execute_query("SELECT name FROM rules WHERE scope = 'network'").unwrap();
    assert_eq!(filtered.columns, vec!["name"]);
    assert_eq!(filtered.rows, vec![vec!["Beta"]]);
    let ordered = registry.execute_query("SELECT name,blacklist FROM rules WHERE blacklist >= 1 ORDER BY blacklist DESC LIMIT 1").unwrap();
    assert_eq!(ordered.rows, vec![vec!["Beta", "2"]]);
}

#[test]
fn registry_rejects_unsupported_queries_instead_of_ignoring_them() {
    let registry = crate::query::N2Registry::from_file(&parse_n2("").unwrap());
    for query in ["DELETE FROM rules", "SELECT * FROM rules_fake", "SELECT missing FROM rules", "SELECT * FROM rules WHERE scope='a' OR scope='b'", "SELECT * FROM rules; DROP TABLE rules"] {
        assert!(registry.execute_query(query).is_err(), "silently ignored query: {query}");
    }
}

#[test]
fn parser_preserves_workflow_contract_references() {
    let ast = parse_n2("@workflow Work { apply_contracts: [Gate] step Run {} }").unwrap();
    let crate::ast::Block::Workflow(workflow) = &ast.blocks[0] else { panic!("workflow"); };
    assert_eq!(workflow.contracts, vec!["Gate"]);
}

#[test]
fn parser_supports_documented_templates_quotes_and_regex_slashes() {
    let ast = parse_n2(r##"@meta { name: """multi
line""" note: "say \"hello\"" } @rule Safe { blacklist: [/foo\/bar/i] }"##).unwrap();
    let crate::ast::Block::Meta(meta) = &ast.blocks[0] else { panic!("meta"); };
    assert!(matches!(&meta.fields[0].value, crate::ast::Value::String(v) if v=="multi\nline"));
    assert!(matches!(&meta.fields[1].value, crate::ast::Value::String(v) if v=="say \"hello\""));
    let crate::ast::Block::Rule(rule) = &ast.blocks[1] else { panic!("rule"); };
    assert_eq!(rule.blacklist, vec![r"/foo\/bar/i"]);
}

#[test]
fn parser_keeps_arrow_text_inside_condition_values() {
    let ast = parse_n2(r#"@contract Gate { precondition { $input.value == "a=>b" => "keep arrow" } }"#).unwrap();
    let crate::ast::Block::Contract(contract) = &ast.blocks[0] else { panic!("contract"); };
    assert_eq!(contract.preconditions[0].expression, r#"$input.value == "a=>b""#);
    assert_eq!(contract.preconditions[0].message, "keep arrow");
}

#[test]
fn parser_rejects_malformed_blacklist_instead_of_treating_it_as_metadata() {
    assert!(parse_n2(r#"@rule Safe { blacklist: ["blocked"] }"#).is_err());
}
