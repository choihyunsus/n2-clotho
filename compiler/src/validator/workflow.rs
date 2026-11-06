// Validate workflow references and dependency cycles without recursive traversal.
use super::*;
use std::collections::HashSet;

pub(super) fn validate(file: &N2File, errors: &mut Vec<ValidationError>) {
    let contracts: HashSet<_> = file.blocks.iter().filter_map(|b| match b {
        Block::Contract(c) => Some(c.name.as_str()), _ => None,
    }).collect();
    for block in &file.blocks {
        let Block::Workflow(workflow) = block else { continue; };
        let block = format!("@workflow {}", workflow.name);
        if workflow.name.is_empty() { error(errors, &block, "name", "Workflow name is required"); }
        if workflow.steps.is_empty() { error(errors, &block, "steps", "At least one step is required"); }
        let mut names = HashSet::new();
        for step in &workflow.steps {
            if !names.insert(step.name.as_str()) {
                error(errors, &block, &step.name, "중복된 step 이름");
            }
        }
        for contract in &workflow.contracts {
            if !contracts.contains(contract.as_str()) {
                error(errors, &block, "apply_contracts", &format!("Unknown contract: {contract}"));
            }
        }
        let dependencies = dependencies(workflow, &names, errors);
        if has_cycle(&dependencies) { error(errors, &block, "depends_on", "Cyclic step dependency"); }
    }
}

fn dependencies<'a>(workflow: &'a WorkflowBlock, names: &HashSet<&str>, errors: &mut Vec<ValidationError>) -> HashMap<&'a str, &'a str> {
    let mut dependencies = HashMap::new();
    let block = format!("@workflow {}", workflow.name);
    for step in &workflow.steps {
        let mut seen = false;
        for field in step.fields.iter().filter(|f| f.key == "depends_on") {
            if seen { error(errors, &block, &step.name, "Duplicate depends_on field"); }
            seen = true;
            let dependency = match &field.value {
                Value::String(name) | Value::Identifier(name) if !name.is_empty() => name,
                _ => { error(errors, &block, &step.name, "depends_on must name one step"); continue; }
            };
            if !names.contains(dependency.as_str()) {
                error(errors, &block, &step.name, &format!("Unknown depends_on step: {dependency}"));
            } else {
                dependencies.insert(step.name.as_str(), dependency.as_str());
            }
        }
    }
    dependencies
}

// Iterative graph traversal avoids recursive stack growth on long workflows.
fn has_cycle(dependencies: &HashMap<&str, &str>) -> bool {
    let mut completed = HashSet::new();
    for start in dependencies.keys() {
        let mut active = HashSet::new();
        let mut current = *start;
        while !completed.contains(current) {
            if !active.insert(current) { return true; }
            let Some(next) = dependencies.get(current) else { break; };
            current = next;
        }
        completed.extend(active);
    }
    false
}
