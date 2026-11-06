// .n2 parser — converts pest PEG parse results into N2AST
use pest::Parser;
use pest_derive::Parser;

use super::ast::*;
mod schema;
mod contract;
use schema::parse_schema_block;
use contract::parse_contract_block;

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
pub struct N2Parser;

/// Parse .n2 source code and return an N2File AST
pub fn parse_n2(source: &str) -> Result<N2File, String> {
    let pairs = N2Parser::parse(Rule::n2_file, source)
        .map_err(|e| format!("파싱 에러: {}", e))?;

    let mut blocks = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::n2_file => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::block => {
                            let block_inner = inner_pair.into_inner().next().unwrap();
                            let block = parse_block(block_inner)?;
                            blocks.push(block);
                        }
                        Rule::EOI => {}
                        _ => {}
                    }
                }
            }
            Rule::block => {
                let inner = pair.into_inner().next().unwrap();
                let block = parse_block(inner)?;
                blocks.push(block);
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(N2File { blocks })
}

fn parse_block(pair: pest::iterators::Pair<Rule>) -> Result<Block, String> {
    match pair.as_rule() {
        Rule::meta_block => parse_meta_block(pair).map(Block::Meta),
        Rule::import_block => parse_import_block(pair).map(Block::Import),
        Rule::schema_block => parse_schema_block(pair).map(Block::Schema),
        Rule::contract_block => parse_contract_block(pair).map(Block::Contract),
        Rule::rule_block => parse_rule_block(pair).map(Block::Rule),
        Rule::workflow_block => parse_workflow_block(pair).map(Block::Workflow),
        Rule::query_block => parse_query_block(pair).map(Block::Query),
        Rule::semantic_block => parse_semantic_block(pair).map(Block::Semantic),
        Rule::recipe_block => parse_recipe_block(pair).map(Block::Recipe),
        _ => Err(format!("알 수 없는 블록 타입: {:?}", pair.as_rule())),
    }
}

// ── @meta parsing ──
fn parse_meta_block(pair: pest::iterators::Pair<Rule>) -> Result<MetaBlock, String> {
    let mut fields = Vec::new();
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::meta_field {
            fields.push(parse_field(inner)?);
        }
    }
    Ok(MetaBlock { fields })
}

// ── @import parsing ──
fn parse_import_block(pair: pest::iterators::Pair<Rule>) -> Result<ImportBlock, String> {
    let mut imports = Vec::new();
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::import_stmt {
            let mut from = String::new();
            let mut names = Vec::new();
            for part in inner.into_inner() {
                match part.as_rule() {
                    Rule::string_literal => from = strip_quotes(part.as_str()),
                    Rule::import_list => {
                        for id in part.into_inner() {
                            if id.as_rule() == Rule::identifier {
                                names.push(id.as_str().to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
            imports.push(ImportStmt { from, names });
        }
    }
    Ok(ImportBlock { imports })
}

// ── @rule parsing ──
fn parse_rule_block(pair: pest::iterators::Pair<Rule>) -> Result<RuleBlock, String> {
    let mut name = String::new();
    let mut fields = Vec::new();
    let mut checks = Vec::new();
    let mut blacklist = Vec::new();
    let mut on_match = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => name = inner.as_str().to_string(),
            Rule::rule_body => {
                let body = inner.into_inner().next().unwrap();
                match body.as_rule() {
                    Rule::rule_field => fields.push(parse_nonreserved_field(body, &["blacklist", "check", "on_match"])?),
                    Rule::check_block => {
                        for stmt in body.into_inner() {
                            if stmt.as_rule() == Rule::condition_stmt {
                                checks.push(parse_condition_stmt(stmt));
                            }
                        }
                    }
                    Rule::blacklist_block => {
                        for regex in body.into_inner() {
                            if regex.as_rule() == Rule::regex_list {
                                for r in regex.into_inner() {
                                    blacklist.push(r.as_str().to_string());
                                }
                            }
                        }
                    }
                    Rule::on_match_block => {
                        on_match.push(body.as_str().to_string());
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(RuleBlock { name, fields, checks, blacklist, on_match })
}

// ── @workflow parsing ──
fn parse_workflow_block(pair: pest::iterators::Pair<Rule>) -> Result<WorkflowBlock, String> {
    let mut name = String::new();
    let mut fields = Vec::new();
    let mut contracts = Vec::new();
    let mut steps = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => name = inner.as_str().to_string(),
            Rule::workflow_body => {
                let body = inner.into_inner().next().unwrap();
                match body.as_rule() {
                    Rule::workflow_field => fields.push(parse_nonreserved_field(body, &["apply_contracts"])?),
                    Rule::apply_contracts => {
                        for part in body.into_inner() {
                            if part.as_rule() == Rule::identifier_list {
                                for id in part.into_inner() {
                                    contracts.push(id.as_str().to_string());
                                }
                            }
                        }
                    }
                    Rule::step_block => {
                        steps.push(parse_step_block(body)?);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(WorkflowBlock { name, fields, contracts, steps })
}

fn parse_step_block(pair: pest::iterators::Pair<Rule>) -> Result<StepBlock, String> {
    let mut name = String::new();
    let mut fields = Vec::new();
    let mut output = None;
    let mut expects = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => name = inner.as_str().to_string(),
            Rule::step_body => {
                let body = inner.into_inner().next().unwrap();
                match body.as_rule() {
                    Rule::step_field => fields.push(parse_field(body)?),
                    Rule::output_binding => {
                        for part in body.into_inner() {
                            if part.as_rule() == Rule::variable {
                                output = Some(part.as_str().to_string());
                            }
                        }
                    }
                    Rule::expect_block => {
                        for stmt in body.into_inner() {
                            if stmt.as_rule() == Rule::expect_stmt {
                                let (condition, action) = condition_parts(&stmt);
                                expects.push(ExpectStmt {
                                    condition,
                                    action,
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(StepBlock { name, fields, output, expects })
}

// ── @recipe parsing ──
fn parse_recipe_block(pair: pest::iterators::Pair<Rule>) -> Result<RecipeBlock, String> {
    let mut name = String::new();
    let mut fields = Vec::new();
    let mut steps = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => name = inner.as_str().to_string(),
            Rule::recipe_body => {
                let body = inner.into_inner().next().unwrap();
                match body.as_rule() {
                    Rule::recipe_field => fields.push(parse_field(body)?),
                    Rule::step_block => steps.push(parse_step_block(body)?),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(RecipeBlock { name, fields, steps })
}

// ── @query parsing ──
fn parse_query_block(pair: pest::iterators::Pair<Rule>) -> Result<QueryBlock, String> {
    let mut name = String::new();
    let mut fields = Vec::new();
    let mut sql = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => name = inner.as_str().to_string(),
            Rule::query_body => {
                let body = inner.into_inner().next().unwrap();
                match body.as_rule() {
                    Rule::query_field => fields.push(parse_field(body)?),
                    Rule::sql_block => {
                        for part in body.into_inner() {
                            if part.as_rule() == Rule::sql_content {
                                sql = Some(part.as_str().trim().to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(QueryBlock { name, fields, sql })
}

// ── @semantic parsing ──
fn parse_semantic_block(pair: pest::iterators::Pair<Rule>) -> Result<SemanticBlock, String> {
    let mut name = String::new();
    let mut fields = Vec::new();
    let mut indexes = Vec::new();
    let mut aliases = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => name = inner.as_str().to_string(),
            Rule::semantic_body => {
                let body = inner.into_inner().next().unwrap();
                match body.as_rule() {
                    Rule::semantic_field => fields.push(parse_field(body)?),
                    Rule::index_block => {
                        let mut idx_name = String::new();
                        let mut idx_fields = Vec::new();
                        for part in body.into_inner() {
                            match part.as_rule() {
                                Rule::identifier => idx_name = part.as_str().to_string(),
                                Rule::step_field => idx_fields.push(parse_field(part)?),
                                _ => {}
                            }
                        }
                        indexes.push(IndexDef { name: idx_name, fields: idx_fields });
                    }
                    Rule::aliases_block => {
                        for stmt in body.into_inner() {
                            if stmt.as_rule() == Rule::alias_stmt {
                                let parts: Vec<_> = stmt.into_inner().collect();
                                if parts.len() >= 2 {
                                    aliases.push(AliasDef {
                                        pattern: strip_quotes(parts[0].as_str()),
                                        target: parts[1].as_str().to_string(),
                                    });
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(SemanticBlock { name, fields, indexes, aliases })
}

// ── Common helpers ──
fn parse_field(pair: pest::iterators::Pair<Rule>) -> Result<Field, String> {
    let mut key = String::new();
    let mut val = Value::String(String::new());

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identifier => key = inner.as_str().to_string(),
            Rule::value => val = parse_value(inner),
            _ => {}
        }
    }

    Ok(Field { key, value: val })
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> Value {
    let inner = pair.into_inner().next();
    match inner {
        Some(p) => match p.as_rule() {
            Rule::string_literal | Rule::template_string => Value::String(strip_quotes(p.as_str())),
            Rule::number => Value::Number(p.as_str().parse().unwrap_or(0.0)),
            Rule::boolean => Value::Bool(p.as_str() == "true"),
            Rule::identifier => Value::Identifier(p.as_str().to_string()),
            Rule::variable => Value::String(p.as_str().to_string()),
            Rule::duration => Value::String(p.as_str().to_string()),
            Rule::regex_literal => Value::String(p.as_str().to_string()),
            Rule::array_literal => {
                let items: Vec<Value> = p.into_inner()
                    .filter(|v| v.as_rule() == Rule::value)
                    .map(parse_value)
                    .collect();
                Value::Array(items)
            }
            Rule::function_call => Value::String(p.as_str().to_string()),
            _ => Value::String(p.as_str().to_string()),
        },
        None => Value::String(String::new()),
    }
}

fn parse_nonreserved_field(pair: pest::iterators::Pair<Rule>, reserved: &[&str]) -> Result<Field, String> {
    let field = parse_field(pair)?;
    if reserved.contains(&field.key.as_str()) {
        return Err(format!("Invalid syntax for reserved construct '{}'", field.key));
    }
    Ok(field)
}

fn condition_parts(pair: &pest::iterators::Pair<Rule>) -> (String, String) {
    let expression = pair.clone().into_inner().next().expect("condition expression");
    let offset = expression.as_span().end() - pair.as_span().start();
    let action = pair.as_str()[offset..].trim_start().strip_prefix("=>").unwrap_or_default().trim();
    (expression.as_str().trim().to_string(), action.to_string())
}

fn parse_condition_stmt(pair: pest::iterators::Pair<Rule>) -> ConditionStmt {
    let (expression, message) = condition_parts(&pair);
    ConditionStmt {
        expression,
        message: strip_quotes(&message),
    }
}

fn strip_quotes(s: &str) -> String {
    let s = s.trim();
    if s.starts_with("\"\"\"") && s.ends_with("\"\"\"") {
        s[3..s.len()-3].to_string()
    } else if s.starts_with('"') && s.ends_with('"') {
        let mut result = String::new();
        let mut chars = s[1..s.len()-1].chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\\' && chars.peek().is_some_and(|next| matches!(next, '"' | '\\')) {
                result.push(chars.next().expect("peeked escape"));
            } else { result.push(ch); }
        }
        result
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_meta() {
        let source = r#"
@meta {
  name: "test"
  version: "1.0.0"
  enforce: strict
}
"#;
        let result = parse_n2(source);
        assert!(result.is_ok(), "parse failed: {:?}", result.err());
        let file = result.unwrap();
        assert_eq!(file.blocks.len(), 1);
    }

    #[test]
    fn test_parse_workflow() {
        let source = r#"
@meta {
  name: "test-workflow"
  target: workflow
  enforce: strict
}

@workflow Boot {
  trigger: session_start
  enforce: strict

  step boot {
    action: n2_boot()
    required: true
    timeout: 15s
    output -> $BOOT
  }

  step greet {
    depends_on: boot
    action: compose_response()
  }
}
"#;
        let result = parse_n2(source);
        assert!(result.is_ok(), "parse failed: {:?}", result.err());
        let file = result.unwrap();
        assert_eq!(file.blocks.len(), 2);
    }
}
