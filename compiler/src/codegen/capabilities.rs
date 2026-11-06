// Standalone code generation must not silently omit executable constraints.
use super::{CodegenError, N2File};
use crate::{ast::*, validator::{self, Severity}};
use std::collections::HashSet;

pub(super) fn validate(ast: &N2File, target: &str) -> Result<(), CodegenError> {
    if let Some(error) = validator::validate(ast).into_iter().find(|e| e.severity == Severity::Error) {
        return Err(fail(target, format!("Invalid source: {error}")));
    }
    for block in &ast.blocks {
        match block {
            Block::Meta(_) => {}
            Block::Contract(contract) => validate_contract(contract, target)?,
            Block::Rule(rule) => {
                identifier(&rule.name, target)?;
                if !rule.checks.is_empty() || !rule.on_match.is_empty() {
                    return Err(fail(target, "Rule check/on_match expressions are not implemented by standalone generators; only blacklist matching is supported"));
                }
            }
            Block::Schema(schema) => validate_schema(schema, ast, target)?,
            Block::Workflow(workflow) => {
                identifier(&workflow.name, target)?;
                if !workflow.contracts.is_empty() || workflow.steps.iter().any(|s|
                    !s.fields.is_empty() || s.output.is_some() || !s.expects.is_empty()) {
                    return Err(fail(target, "Standalone generators do not execute workflow steps; only step-name metadata is supported"));
                }
            }
            Block::Import(_) | Block::Query(_) | Block::Semantic(_) | Block::Recipe(_) => {
                return Err(fail(target, "Import/query/semantic/recipe blocks are not implemented by standalone generators and cannot be silently omitted"));
            }
        }
    }
    super::symbols::validate(ast, target)
}

fn validate_contract(contract: &ContractBlock, target: &str) -> Result<(), CodegenError> {
    identifier(&contract.name, target)?;
    if !contract.preconditions.is_empty() || !contract.postconditions.is_empty()
        || !contract.invariants.is_empty() || !contract.recovery.is_empty() {
        return Err(fail(target, "Conditional contracts are not implemented by standalone generators; only state/event transitions are supported"));
    }
    let mut events = HashSet::new();
    for transition in &contract.transitions {
        identifier(&transition.from, target)?;
        identifier(&transition.to, target)?;
        if !events.insert((&transition.from, &transition.on_event)) {
            return Err(fail(target, "Duplicate state/event transitions are ambiguous"));
        }
    }
    Ok(())
}

fn validate_schema(schema: &SchemaBlock, ast: &N2File, target: &str) -> Result<(), CodegenError> {
    if target != "rust" || schema.definitions.iter().any(|d| !d.is_enum) {
        return Err(fail(target, "Standalone schema output supports Rust enums only; structured field validation is not implemented"));
    }
    let mut names = HashSet::new();
    for definition in &schema.definitions {
        identifier(&definition.name, target)?;
        if !names.insert(&definition.name) || schema_name_collision(&definition.name, ast) {
            return Err(fail(target, "Schema name collides with another generated type"));
        }
        let mut fields = HashSet::new();
        for field in &definition.fields {
            identifier(&field.name, target)?;
            if !fields.insert(&field.name) || field.type_expr.is_some() || !field.attributes.is_empty() {
                return Err(fail(target, "Enum variants must be unique names without field constraints"));
            }
        }
    }
    Ok(())
}

fn schema_name_collision(name: &str, ast: &N2File) -> bool {
    matches!(name, "HashMap" | "ContractViolation") || ast.blocks.iter().any(|block| match block {
        Block::Contract(contract) => name == contract.name || name == format!("{}State", contract.name),
        Block::Rule(rule) => name == format!("{}Rule", rule.name),
        _ => false,
    })
}

fn identifier(value: &str, target: &str) -> Result<(), CodegenError> {
    // Use the intersection of identifier syntax and reserved words across targets.
    let reserved = "as async await auto bool break case catch char class const continue crate default def defer delete do double dyn else enum except explicit export extern false final finally float fn for friend from func global go goto if impl import in inline int interface is lambda let long loop map match mod move mut namespace new nil none not null operator or override package pass private protected pub public raise range ref register return rune select self short signed sizeof static str struct super switch template this throw trait true try type typedef typename union unsigned unsafe use using var virtual void volatile where while with yield";
    let valid = value != "_" && value.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !reserved.split_whitespace().any(|word| word.eq_ignore_ascii_case(value));
    if valid { Ok(()) } else { Err(fail(target, format!("Nonportable generated identifier: {value}"))) }
}

fn fail(target: &str, message: impl Into<String>) -> CodegenError {
    CodegenError { target: target.into(), message: message.into() }
}
