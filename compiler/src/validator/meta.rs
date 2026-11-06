// Validate required and unambiguous file metadata.
use super::*;
use std::collections::HashSet;

pub(super) fn validate(file: &N2File, errors: &mut Vec<ValidationError>) {
    let blocks: Vec<_> = file.blocks.iter().filter_map(|block| match block {
        Block::Meta(meta) => Some(meta), _ => None,
    }).collect();
    let Some(meta) = blocks.first() else {
        error(errors, "@meta", "(missing)", "@meta 블록은 필수입니다");
        return;
    };
    if blocks.len() > 1 { error(errors, "@meta", "(duplicate)", "@meta 블록은 하나만 허용됩니다"); }
    let mut names = HashSet::new();
    for field in &meta.fields {
        if !names.insert(field.key.as_str()) { error(errors, "@meta", &field.key, "Duplicate metadata field"); }
    }
    for required in ["name", "enforce"] {
        if !names.contains(required) { error(errors, "@meta", required, "필수 필드가 누락되었습니다"); }
    }
    if let Some(field) = meta.fields.iter().find(|f| f.key == "name") {
        if !matches!(&field.value, Value::String(v) | Value::Identifier(v) if !v.trim().is_empty()) {
            error(errors, "@meta", "name", "Name must be nonempty text");
        }
    }
    if let Some(field) = meta.fields.iter().find(|f| f.key == "enforce") {
        if !matches!(&field.value, Value::String(v) | Value::Identifier(v) if ["strict", "warn", "passive"].contains(&v.as_str())) {
            error(errors, "@meta", "enforce", "strict|warn|passive 중 하나여야 합니다");
        }
    }
}
