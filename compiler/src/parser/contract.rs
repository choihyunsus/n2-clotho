// Parse contract clauses without overwriting state declarations.
use super::*;

pub(super) fn parse_contract_block(pair: pest::iterators::Pair<Rule>) -> Result<ContractBlock, String> {
    let mut contract = ContractBlock { name: String::new(), fields: Vec::new(), preconditions: Vec::new(),
        postconditions: Vec::new(), invariants: Vec::new(), recovery: Vec::new(), states: None, transitions: Vec::new() };
    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::identifier => contract.name = part.as_str().to_string(),
            Rule::contract_body => {
                let body = part.into_inner().next().ok_or("Missing contract body")?;
                parse_body(body, &mut contract)?;
            }
            _ => {}
        }
    }
    Ok(contract)
}

fn parse_body(body: pest::iterators::Pair<Rule>, contract: &mut ContractBlock) -> Result<(), String> {
    match body.as_rule() {
        Rule::contract_field => contract.fields.push(parse_nonreserved_field(body,
            &["states", "precondition", "postcondition", "invariant", "recovery", "transitions"])?),
        Rule::precondition_block => contract.preconditions.extend(conditions(body)),
        Rule::postcondition_block => contract.postconditions.extend(conditions(body)),
        Rule::invariant_block => contract.invariants.extend(body.into_inner().map(|p| p.as_str().to_string())),
        Rule::recovery_block => contract.recovery.extend(body.into_inner().map(|p| p.as_str().to_string())),
        Rule::states_block => {
            if contract.states.is_some() { return Err("Duplicate states declaration".into()); }
            contract.states = body.into_inner().find(|p| p.as_rule() == Rule::identifier).map(|p| p.as_str().to_string());
        }
        Rule::transitions_block => {
            for statement in body.into_inner().filter(|p| p.as_rule() == Rule::transition_stmt) {
                let mut parts = statement.into_inner();
                let from = parts.next().ok_or("Missing transition source")?.as_str().to_string();
                let to = parts.next().ok_or("Missing transition target")?.as_str().to_string();
                let on_event = parts.next().ok_or("Missing transition event")?.as_str().to_string();
                contract.transitions.push(TransitionStmt { from, to, on_event });
            }
        }
        _ => {}
    }
    Ok(())
}

fn conditions(pair: pest::iterators::Pair<Rule>) -> Vec<ConditionStmt> {
    pair.into_inner().filter(|p| p.as_rule() == Rule::condition_stmt).map(parse_condition_stmt).collect()
}
