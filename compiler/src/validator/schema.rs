// Validate schema structure; runtime type semantics belong to consumers.
use super::*;
use std::collections::HashSet;

pub(super) fn validate(file: &N2File, errors: &mut Vec<ValidationError>) {
    let mut names = HashSet::new();
    for block in &file.blocks {
        let Block::Schema(schema) = block else { continue; };
        for definition in &schema.definitions {
            if !names.insert(&definition.name) {
                error(errors, "@schema", &definition.name, "Duplicate schema name");
            }
            let block = format!("@schema {}", definition.name);
            let mut fields = HashSet::new();
            for field in &definition.fields {
                if !fields.insert(&field.name) {
                    error(errors, &block, &field.name, "Duplicate schema field");
                }
                if definition.is_enum && (field.type_expr.is_some() || !field.attributes.is_empty()) {
                    error(errors, &block, &field.name, "Enum variants cannot have types or constraints");
                }
                if !definition.is_enum && field.type_expr.is_none() {
                    error(errors, &block, &field.name, "Object fields require a type");
                }
            }
        }
    }
}
