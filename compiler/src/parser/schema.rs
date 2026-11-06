// Preserve explicit enum declarations and schema fields from the PEG grammar.
use super::*;

pub(super) fn parse_schema_block(pair: pest::iterators::Pair<Rule>) -> Result<SchemaBlock, String> {
    let definitions = pair.into_inner().filter(|p| p.as_rule() == Rule::schema_def)
        .map(parse_definition).collect();
    Ok(SchemaBlock { definitions })
}

fn parse_definition(pair: pest::iterators::Pair<Rule>) -> SchemaDef {
    let mut definition = SchemaDef { name: String::new(), is_enum: false, fields: Vec::new() };
    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::identifier => definition.name = part.as_str().to_string(),
            Rule::enum_marker => definition.is_enum = true,
            Rule::schema_field => definition.fields.push(parse_schema_field(part)),
            _ => {}
        }
    }
    definition
}

fn parse_schema_field(pair: pest::iterators::Pair<Rule>) -> SchemaField {
    let mut field = SchemaField { name: String::new(), type_expr: None, attributes: Vec::new() };
    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::identifier => field.name = part.as_str().to_string(),
            Rule::type_expr => field.type_expr = Some(part.as_str().to_string()),
            Rule::field_attrs => field.attributes.extend(part.into_inner().map(|p| p.as_str().to_string())),
            _ => {}
        }
    }
    field
}
