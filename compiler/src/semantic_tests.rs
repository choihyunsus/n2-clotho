// Exercise parsed declarations, generated code and invalid simulation behavior.
use super::{ast::*, codegen::*, contract::*, parser::parse_n2, validator::*};

fn parse(body: &str) -> N2File {
    parse_n2(&format!("@meta {{ name: \"SemanticProbe\" enforce: strict }} {body}")).unwrap()
}

fn invalid(body: &str) {
    let errors = validate(&parse(body));
    assert!(errors.iter().any(|error| error.severity == Severity::Error), "Accepted: {body}");
}

#[test]
fn enum_marker_survives_parsing() {
    let file = parse("@schema { Mode = enum { Idle, Done } }");
    let Block::Schema(schema) = &file.blocks[1] else { panic!("schema"); };
    assert!(schema.definitions[0].is_enum);
}

#[test]
fn declared_enum_reaches_rust_generator() {
    let file = parse("@schema { Mode = enum { Idle, Done } }");
    let meta = CompilationMeta { source_name: "Probe".into(), source_version: "1".into(),
        target: "rust".into(), extension: ".rs".into(), compiler_version: "test".into(), compiled_at: "test".into() };
    let code = BackendRegistry::new().compile(&file, "rust", &meta).unwrap();
    assert!(code.contains("pub enum Mode"));
}

#[test]
fn duplicate_state_declarations_are_not_overwritten() {
    assert!(parse_n2("@contract Gate { states: First states: Second }").is_err());
}

#[test]
fn undeclared_transition_state_is_invalid() {
    invalid("@schema { Mode = enum { Idle, Done } } @contract Gate { states: Mode transitions { Idle -> Typo : on go } }");
}

#[test]
fn ambiguous_transition_is_invalid_before_simulation() {
    invalid("@contract Gate { states: Mode transitions { Idle -> Ready : on go Idle -> Done : on go } }");
}

#[test]
fn simulator_does_not_choose_the_first_ambiguous_transition() {
    let mut runtime = ContractRuntime::from_file(&parse("@contract Gate { states: Mode transitions { Idle -> Ready : on go Idle -> Done : on go } }"));
    assert!(runtime.simulate_event("Gate", "go").is_err());
    assert_eq!(runtime.current_states["Gate"], "Idle");
}

#[test]
fn simulator_keeps_declared_unreachable_states() {
    let runtime = ContractRuntime::from_file(&parse("@schema { Mode = enum { Idle, Done, Orphan } } @contract Gate { states: Mode transitions { Idle -> Done : on go } }"));
    assert_eq!(runtime.machines["Gate"].states, ["Idle", "Done", "Orphan"]);
    assert!(runtime.check_integrity().iter().any(|v| matches!(v.violation_type, ViolationType::UnreachableState) && v.message.contains("Orphan")));
}

#[test]
fn simulator_rejects_missing_target_without_mutating_state() {
    let mut runtime = ContractRuntime::from_file(&parse("@schema { Mode = enum { Idle, Done } } @contract Gate { states: Mode transitions { Idle -> Typo : on go } }"));
    assert!(runtime.check_integrity().iter().any(|v| matches!(v.violation_type, ViolationType::MissingState)));
    assert!(runtime.simulate_event("Gate", "go").is_err());
    assert_eq!(runtime.current_states["Gate"], "Idle");
}

#[test]
fn simulator_rejects_duplicate_contract_names() {
    let mut runtime = ContractRuntime::from_file(&parse("@contract Gate { states: Mode transitions { Idle -> Done : on go } } @contract Gate { states: Mode transitions { Idle -> Ready : on go } }"));
    assert!(runtime.simulate_event("Gate", "go").is_err());
}

#[test]
fn simulator_does_not_claim_to_evaluate_conditions() {
    let mut runtime = ContractRuntime::from_file(&parse("@contract Gate { states: Mode precondition { $input.ready == true => \"required\" } transitions { Idle -> Done : on go } }"));
    assert!(runtime.simulate_event("Gate", "go").is_err());
}

#[test]
fn duplicate_schema_names_are_invalid_across_blocks() {
    invalid("@schema { Mode = enum { Idle } } @schema { Mode = enum { Done } }");
}

#[test]
fn duplicate_schema_fields_are_invalid() {
    invalid("@schema { Input { path: String path: String } }");
}

#[test]
fn enum_variants_cannot_be_typed() {
    invalid("@schema { Mode = enum { Idle: String } }");
}

#[test]
fn state_reference_cannot_point_at_an_object_schema() {
    invalid("@schema { Mode { path: String } } @contract Gate { states: Mode transitions { Idle -> Done : on go } }");
}

#[test]
fn workflow_dependencies_reject_wrong_types_and_cycles() {
    invalid("@workflow Work { step A { depends_on: 4 } }");
    invalid("@workflow Work { step A { depends_on: A } }");
    invalid("@workflow Work { step A { depends_on: B } step B { depends_on: A } }");
}

#[test]
fn workflow_contracts_must_resolve() {
    invalid("@workflow Work { apply_contracts: [Missing] step A {} }");
}

#[test]
fn legacy_state_names_and_valid_dependencies_remain_usable() {
    let file = parse("@contract Gate { states: Legacy transitions { Idle -> Done : on go } } @workflow Work { apply_contracts: [Gate] step A {} step B { depends_on: A } }");
    assert!(!validate(&file).iter().any(|e| e.severity == Severity::Error));
    let mut runtime = ContractRuntime::from_file(&file);
    assert_eq!(runtime.simulate_event("Gate", "go").unwrap(), "Done");
    assert!(runtime.simulate_event("Gate", "go").is_err());
    assert_eq!(runtime.current_states["Gate"], "Done");
}

#[test]
fn metadata_rejects_ambiguous_or_nontext_names() {
    for source in ["@meta { name: 4 enforce: strict }", "@meta { name: \" \" enforce: strict }",
        "@meta { name: \"A\" name: \"B\" enforce: strict }"] {
        assert!(validate(&parse_n2(source).unwrap()).iter().any(|e| e.severity == Severity::Error));
    }
}

#[test]
fn object_fields_require_types() {
    invalid("@schema { Input { path } }");
}

#[test]
fn local_enum_uses_first_transition_as_initial_state() {
    let file = parse("@schema { Mode = enum { Done, Idle } } @contract Gate { states: Mode transitions { Idle -> Done : on go } }");
    assert!(!validate(&file).iter().any(|e| e.severity == Severity::Error));
    let mut runtime = ContractRuntime::from_file(&file);
    assert_eq!(runtime.current_states["Gate"], "Idle");
    assert_eq!(runtime.simulate_event("Gate", "go").unwrap(), "Done");
}

#[test]
fn release_codegen_export_fixtures_enum() {
    let file = parse("@schema { Mode = enum { Idle, Done } } @contract Gate { states: Mode transitions { Idle -> Done : on go } }");
    let meta = CompilationMeta { source_name: "EnumProbe".into(), source_version: "1".into(),
        target: "rust".into(), extension: ".rs".into(), compiler_version: "test".into(), compiled_at: "test".into() };
    let code = BackendRegistry::new().compile(&file, "rust", &meta).unwrap();
    if let Ok(directory) = std::env::var("N2_RELEASE_CODEGEN_DIR") {
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(std::path::Path::new(&directory).join("enum.rs"), code).unwrap();
    }
}

#[test]
fn generated_enum_names_cannot_collide_with_runtime_symbols() {
    let meta = CompilationMeta { source_name: "Probe".into(), source_version: "1".into(),
        target: "rust".into(), extension: ".rs".into(), compiler_version: "test".into(), compiled_at: "test".into() };
    for name in ["HashMap", "ContractViolation", "Gate", "GateState", "SafeRule"] {
        let file = parse(&format!("@schema {{ {name} = enum {{ Idle }} }} @contract Gate {{ states: Mode transitions {{ Idle -> Done : on go }} }} @rule Safe {{ blacklist: [/blocked/] }}"));
        assert!(BackendRegistry::new().compile(&file, "rust", &meta).is_err(), "Colliding type: {name}");
    }
}
