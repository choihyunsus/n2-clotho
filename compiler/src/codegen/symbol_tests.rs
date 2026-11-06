// Reject source names that collide only after target-language code generation.
use super::*;
use crate::parser::parse_n2;

fn contract(name: &str) -> String {
    format!("@contract {name} {{ states: Mode transitions {{ Idle -> Done : on go }} }}")
}

fn meta() -> CompilationMeta {
    CompilationMeta { source_name: "Symbols".into(), source_version: "1".into(), target: String::new(),
        extension: String::new(), compiler_version: "test".into(), compiled_at: "test".into() }
}

fn rejected(target: &str, body: &str) {
    let ast = parse_n2(&format!("@meta {{ name: \"Symbols\" enforce: strict }} {body}")).unwrap();
    assert!(BackendRegistry::new().compile(&ast, target, &meta()).is_err(), "{target} accepted colliding declarations: {body}");
}

#[test]
fn c_rejects_folded_names_and_enum_constants() {
    rejected("c", &(contract("Gate") + &contract("gate")));
    rejected("c", "@contract Gate { states: Mode transitions { X_Y -> Done : on go } } @contract Gate_X { states: Mode transitions { Y -> Done : on go } }");
}

#[test]
fn cpp_rejects_generated_and_runtime_type_collisions() {
    rejected("cpp", &(contract("Gate") + &contract("GateState")));
    rejected("cpp", &contract("ContractViolation"));
    rejected("cpp", &contract("currentState"));
    rejected("cpp", &contract("transition"));
}

#[test]
fn rust_rejects_generated_and_runtime_type_collisions() {
    rejected("rust", &(contract("Gate") + &contract("GateState")));
    rejected("rust", &contract("ContractViolation"));
    rejected("rust", &(contract("SafeRule") + "@rule Safe { blacklist: [/blocked/] }"));
}

#[test]
fn go_rejects_constructor_and_import_collisions() {
    rejected("go", &(contract("Gate") + &contract("NewGate")));
    rejected("go", &contract("fmt"));
}

#[test]
fn python_rejects_shared_transition_maps_and_runtime_names() {
    rejected("python", &(contract("Gate") + &contract("gate")));
    rejected("python", &contract("ContractViolation"));
    rejected("python", &contract("Enum"));
}

#[test]
fn typescript_rejects_shared_transition_maps_and_runtime_names() {
    rejected("ts", &(contract("Gate") + &contract("gate")));
    rejected("ts", &contract("ContractViolation"));
    rejected("ts", "@workflow Build { step Run {} } @workflow build { step Start {} }");
}

#[test]
fn unrelated_declarations_can_generate_together_in_all_targets() {
    let source = format!("@meta {{ name: \"Symbols\" enforce: strict }} {} {} @rule Safe {{ blacklist: [/blocked/] }} @workflow Work {{ step Run {{}} }}", contract("Alpha"), contract("Beta"));
    let ast = parse_n2(&source).unwrap();
    for (target, _, result) in BackendRegistry::new().compile_all(&ast, &meta()) {
        assert!(result.is_ok(), "{target}: {:?}", result.err());
    }
}
