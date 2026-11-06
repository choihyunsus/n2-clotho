// Validate actual generated identifiers, including prefixes and case transforms.
use super::{CodegenError, collect_states};
use crate::ast::*;
use std::collections::HashSet;

pub(super) fn validate(ast: &N2File, target: &str) -> Result<(), CodegenError> {
    let mut names: HashSet<String> = runtime_names(ast, target).iter().map(|s| s.to_string()).collect();
    for block in &ast.blocks {
        if let Block::Contract(contract) = block {
            if target == "cpp" && ["currentState", "transition", "state_"].contains(&contract.name.as_str()) {
                return Err(collision(target, &contract.name));
            }
        }
        for name in symbols(block, target) {
            if !names.insert(name.clone()) { return Err(collision(target, &name)); }
        }
    }
    Ok(())
}

fn collision(target: &str, name: &str) -> CodegenError {
    CodegenError { target: target.into(), message: format!("Generated identifier collides with another declaration or runtime symbol: {name}") }
}

fn runtime_names(ast: &N2File, target: &str) -> Vec<&'static str> {
    match target {
        "rust" => vec!["HashMap", "ContractViolation"],
        "cpp" => vec!["ContractViolation", "std"],
        "python" => vec!["ContractViolation", "Enum", "auto", "Optional", "Tuple", "re", "property", "classmethod", "super", "Exception"],
        "ts" => vec!["ContractViolation", "Error", "Record"],
        "c" => vec!["N2_CONTRACT_H", "N2_OK", "N2_ERR_INVALID_TRANSITION", "N2_ERR_BLACKLISTED", "strcmp", "strstr"],
        "go" => {
            let mut names = Vec::new();
            if ast.blocks.iter().any(|b| matches!(b, Block::Contract(c) if !c.transitions.is_empty())) {
                names.extend(["ContractViolation", "fmt"]);
            }
            if ast.blocks.iter().any(|b| matches!(b, Block::Rule(r) if !r.blacklist.is_empty())) { names.push("regexp"); }
            names
        }
        _ => Vec::new(),
    }
}

fn symbols(block: &Block, target: &str) -> Vec<String> {
    match block {
        Block::Contract(contract) if !contract.transitions.is_empty() => contract_names(contract, target),
        Block::Schema(schema) => schema.definitions.iter().map(|d| d.name.clone()).collect(),
        Block::Rule(rule) if !rule.blacklist.is_empty() || target == "rust" => vec![match target {
            "c" => format!("{}_check_blacklist", rule.name.to_lowercase()),
            "go" => format!("{}CheckBlacklist", rule.name),
            _ => format!("{}Rule", rule.name),
        }],
        Block::Workflow(workflow) => workflow_names(&workflow.name, target),
        _ => Vec::new(),
    }
}

fn contract_names(contract: &ContractBlock, target: &str) -> Vec<String> {
    let name = &contract.name;
    let mut names = match target {
        "c" => {
            let lower = name.to_lowercase();
            vec![format!("{lower}_state_t"), format!("{lower}_transition"), format!("{lower}_state_name")]
        }
        "go" => vec![name.clone(), format!("{name}State"), format!("{name}StateNames"), format!("New{name}")],
        _ => vec![name.clone(), format!("{name}State")],
    };
    match target {
        "c" | "go" => {
            let prefix = if target == "c" { name.to_uppercase() } else { name.clone() };
            names.extend(collect_states(&contract.transitions).iter().map(|s| format!("{prefix}_{s}")));
        }
        "python" => names.push(format!("_{}_TRANSITIONS", name.to_uppercase())),
        "ts" => names.push(format!("{}_TRANSITIONS", name.to_uppercase())),
        _ => {}
    }
    names
}

fn workflow_names(name: &str, target: &str) -> Vec<String> {
    match target {
        "c" => vec![format!("{}_steps", name.to_lowercase()), format!("{}_step_count", name.to_lowercase())],
        "go" => vec![format!("{name}Steps")],
        "ts" => {
            // Capability checks have already restricted identifiers to nonempty ASCII.
            let camel = name[..1].to_lowercase() + &name[1..];
            vec![format!("{camel}Steps")]
        }
        _ => vec![format!("{name}Workflow")],
    }
}
