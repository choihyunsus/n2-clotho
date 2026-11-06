// N2AST — abstract syntax tree node definitions for the .n2 language
use serde::Serialize;

/// Root node representing an entire .n2 file
#[derive(Debug, Serialize)]
pub struct N2File {
    pub blocks: Vec<Block>,
}

/// Top-level block types
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Block {
    Meta(MetaBlock),
    Import(ImportBlock),
    Schema(SchemaBlock),
    Contract(ContractBlock),
    Rule(RuleBlock),
    Workflow(WorkflowBlock),
    Query(QueryBlock),
    Semantic(SemanticBlock),
    Recipe(RecipeBlock),
}

// ── @meta ──
#[derive(Debug, Serialize)]
pub struct MetaBlock {
    pub fields: Vec<Field>,
}

// ── @import ──
#[derive(Debug, Serialize)]
pub struct ImportBlock {
    pub imports: Vec<ImportStmt>,
}

#[derive(Debug, Serialize)]
pub struct ImportStmt {
    pub from: String,
    pub names: Vec<String>,
}

// ── @schema ──
#[derive(Debug, Serialize)]
pub struct SchemaBlock {
    pub definitions: Vec<SchemaDef>,
}

#[derive(Debug, Serialize)]
pub struct SchemaDef {
    pub name: String,
    pub is_enum: bool,
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Serialize)]
pub struct SchemaField {
    pub name: String,
    pub type_expr: Option<String>,
    pub attributes: Vec<String>,
}

// ── @contract ──
#[derive(Debug, Serialize)]
pub struct ContractBlock {
    pub name: String,
    pub fields: Vec<Field>,
    pub preconditions: Vec<ConditionStmt>,
    pub postconditions: Vec<ConditionStmt>,
    pub invariants: Vec<String>,
    pub recovery: Vec<String>,
    pub states: Option<String>,
    pub transitions: Vec<TransitionStmt>,
}

#[derive(Debug, Serialize)]
pub struct ConditionStmt {
    pub expression: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TransitionStmt {
    pub from: String,
    pub to: String,
    pub on_event: String,
}

// ── @rule ──
#[derive(Debug, Serialize)]
pub struct RuleBlock {
    pub name: String,
    pub fields: Vec<Field>,
    pub checks: Vec<ConditionStmt>,
    pub blacklist: Vec<String>,
    pub on_match: Vec<String>,
}

// ── @workflow ──
#[derive(Debug, Serialize)]
pub struct WorkflowBlock {
    pub name: String,
    pub fields: Vec<Field>,
    pub contracts: Vec<String>,
    pub steps: Vec<StepBlock>,
}

#[derive(Debug, Serialize)]
pub struct StepBlock {
    pub name: String,
    pub fields: Vec<Field>,
    pub output: Option<String>,
    pub expects: Vec<ExpectStmt>,
}

#[derive(Debug, Serialize)]
pub struct ExpectStmt {
    pub condition: String,
    pub action: String,
}

// ── @query ──
#[derive(Debug, Serialize)]
pub struct QueryBlock {
    pub name: String,
    pub fields: Vec<Field>,
    pub sql: Option<String>,
}

// ── @semantic ──
#[derive(Debug, Serialize)]
pub struct SemanticBlock {
    pub name: String,
    pub fields: Vec<Field>,
    pub indexes: Vec<IndexDef>,
    pub aliases: Vec<AliasDef>,
}

#[derive(Debug, Serialize)]
pub struct IndexDef {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize)]
pub struct AliasDef {
    pub pattern: String,
    pub target: String,
}

// ── @recipe ──
/// Lifecycle recipe — ordered step sequence enforced by the N2R executor.
/// Phases: boot, work_start, work_loop, work_end, end.
#[derive(Debug, Serialize)]
pub struct RecipeBlock {
    pub name: String,
    pub fields: Vec<Field>,
    pub steps: Vec<StepBlock>,
}

// ── Common ──
#[derive(Debug, Serialize)]
pub struct Field {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Value>),
    Identifier(String),
}
