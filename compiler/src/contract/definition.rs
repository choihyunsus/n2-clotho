// Resolve declared states and validate state/event simulation boundaries.
use super::*;

/// A local enum is authoritative. Legacy external state type names infer endpoints.
pub(crate) fn states(file: &N2File, contract: &ContractBlock) -> Result<Vec<String>, String> {
    let Some(name) = &contract.states else {
        return if contract.transitions.is_empty() { Ok(Vec::new()) } else { Err("Transitions require states".into()) };
    };
    let definitions: Vec<_> = file.blocks.iter().filter_map(|block| match block {
        Block::Schema(schema) => Some(&schema.definitions), _ => None,
    }).flatten().filter(|definition| &definition.name == name).collect();
    if definitions.len() > 1 { return Err(format!("Duplicate state schema: {name}")); }
    let Some(definition) = definitions.first() else {
        return Ok(crate::codegen::collect_states(&contract.transitions));
    };
    if !definition.is_enum { return Err(format!("State schema must be an enum: {name}")); }
    let mut seen = HashSet::new();
    let mut names = Vec::new();
    for field in &definition.fields {
        if !seen.insert(&field.name) || field.type_expr.is_some() || !field.attributes.is_empty() {
            return Err(format!("Invalid or duplicate state variant: {}", field.name));
        }
        names.push(field.name.clone());
    }
    Ok(names)
}

pub(crate) fn issues(file: &N2File, contract: &ContractBlock) -> Vec<ContractViolation> {
    let mut issues = Vec::new();
    let states = match states(file, contract) {
        Ok(states) => states,
        Err(message) => return vec![violation(&contract.name, ViolationType::MissingState, message)],
    };
    let mut events = HashSet::new();
    for transition in &contract.transitions {
        if !events.insert((&transition.from, &transition.on_event)) {
            issues.push(violation(&contract.name, ViolationType::InvalidTransition,
                format!("Ambiguous state/event transition: {} / {}", transition.from, transition.on_event)));
        }
        for state in [&transition.from, &transition.to] {
            if !states.contains(state) {
                issues.push(violation(&contract.name, ViolationType::MissingState,
                    format!("Transition state '{state}' is not declared")));
            }
        }
    }
    issues
}

pub(super) fn violation(name: &str, violation_type: ViolationType, message: impl Into<String>) -> ContractViolation {
    ContractViolation { contract: name.into(), violation_type, message: message.into() }
}

pub(super) fn simulation_issues(file: &N2File, contract: &ContractBlock) -> Vec<ContractViolation> {
    let mut issues = issues(file, contract);
    if file.blocks.iter().filter(|b| matches!(b, Block::Contract(c) if c.name == contract.name)).count() > 1 {
        issues.push(violation(&contract.name, ViolationType::InvalidTransition, "Duplicate contract name"));
    }
    if !contract.preconditions.is_empty() || !contract.postconditions.is_empty()
        || !contract.invariants.is_empty() || !contract.recovery.is_empty() {
        issues.push(violation(&contract.name, ViolationType::InvalidTransition,
            "State/event simulation cannot evaluate conditions, invariants or recovery actions"));
    }
    issues
}
