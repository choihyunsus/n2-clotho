// Schema Validator — type/required field/range validation for @schema blocks
use super::ast::*;
use std::collections::HashMap;
mod meta;
mod schema;
mod workflow;

/// Validation error
#[derive(Debug)]
pub struct ValidationError {
    pub block: String,
    pub field: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = match self.severity {
            Severity::Error => "❌",
            Severity::Warning => "⚠️",
        };
        write!(f, "{} [{}] {}: {}", icon, self.block, self.field, self.message)
    }
}

/// Validate the entire N2File
pub fn validate(file: &N2File) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // 1. Validate required @meta block
    meta::validate(file, &mut errors);

    // 2. Validate schema structure
    schema::validate(file, &mut errors);

    // 3. Validate @workflow blocks
    workflow::validate(file, &mut errors);

    // 4. Validate @contract blocks
    validate_contracts(file, &mut errors);

    // 5. Validate @rule blocks
    validate_rules(file, &mut errors);

    // 6. Validate unique names
    validate_unique_names(file, &mut errors);

    errors
}

fn validate_contracts(file: &N2File, errors: &mut Vec<ValidationError>) {
    for block in &file.blocks {
        let Block::Contract(contract) = block else { continue; };
        let name = format!("@contract {}", contract.name);
        if contract.name.is_empty() { error(errors, &name, "name", "Contract name is required"); }
        for issue in super::contract::definition::issues(file, contract) {
            error(errors, &name, "transitions", &issue.message);
        }
    }
}

fn error(errors: &mut Vec<ValidationError>, block: &str, field: &str, message: &str) {
    errors.push(ValidationError { block: block.into(), field: field.into(),
        message: message.into(), severity: Severity::Error });
}

/// Validate @rule blocks
fn validate_rules(file: &N2File, errors: &mut Vec<ValidationError>) {
    for block in &file.blocks {
        if let Block::Rule(rule) = block {
            if rule.name.is_empty() {
                errors.push(ValidationError {
                    block: "@rule".to_string(),
                    field: "name".to_string(),
                    message: "규칙 이름이 필요합니다".to_string(),
                    severity: Severity::Error,
                });
            }

            // Warn if both check and blacklist are empty
            if rule.checks.is_empty() && rule.blacklist.is_empty() {
                errors.push(ValidationError {
                    block: format!("@rule {}", rule.name),
                    field: "body".to_string(),
                    message: "check 또는 blacklist가 비어있습니다. 규칙이 아무것도 검증하지 않습니다".to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }
}

/// Validate block name uniqueness
fn validate_unique_names(file: &N2File, errors: &mut Vec<ValidationError>) {
    let mut names: HashMap<String, String> = HashMap::new();

    for block in &file.blocks {
        let (block_type, name) = match block {
            Block::Workflow(w) => ("@workflow", w.name.clone()),
            Block::Contract(c) => ("@contract", c.name.clone()),
            Block::Rule(r) => ("@rule", r.name.clone()),
            Block::Query(q) => ("@query", q.name.clone()),
            Block::Semantic(s) => ("@semantic", s.name.clone()),
            _ => continue,
        };

        if name.is_empty() { continue; }

        if let Some(existing_type) = names.get(&name) {
            errors.push(ValidationError {
                block: block_type.to_string(),
                field: "name".to_string(),
                message: format!("이름 '{}'가 {}에서 이미 사용되고 있습니다", name, existing_type),
                severity: Severity::Error,
            });
        } else {
            names.insert(name, block_type.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_n2;

    #[test]
    fn test_missing_meta() {
        let source = r#"
@workflow Test {
  step do_thing {
    action: test()
  }
}
"#;
        let file = parse_n2(source).unwrap();
        let errors = validate(&file);
        assert!(errors.iter().any(|e| e.block == "@meta" && e.severity == Severity::Error));
    }

    #[test]
    fn test_valid_file() {
        let source = r#"
@meta {
  name: "test"
  enforce: strict
}

@workflow Boot {
  step boot {
    action: n2_boot()
  }
}
"#;
        let file = parse_n2(source).unwrap();
        let errors = validate(&file);
        let error_count = errors.iter().filter(|e| e.severity == Severity::Error).count();
        assert_eq!(error_count, 0, "should have no errors: {:?}", errors);
    }

    #[test]
    fn test_duplicate_step_names() {
        let source = r#"
@meta {
  name: "test"
  enforce: strict
}

@workflow Boot {
  step boot {
    action: first()
  }
  step boot {
    action: second()
  }
}
"#;
        let file = parse_n2(source).unwrap();
        let errors = validate(&file);
        assert!(errors.iter().any(|e| e.message.contains("중복된 step 이름")));
    }
}
