// Preserve blacklist regex semantics and encode literals for generated source.
use super::{CodegenError, N2File};
use crate::ast::Block;

pub fn parts(source: &str) -> (&str, &str) {
    if let Some(body) = source.strip_prefix('/') {
        if let Some(end) = body.rfind('/') { return (&body[..end], &body[end + 1..]); }
    }
    (source, "")
}

pub fn expression(source: &str) -> String {
    let (body, flags) = parts(source);
    let flags: String = flags.chars().filter(|c| *c != 'g').collect();
    if flags.is_empty() { body.into() } else { format!("(?{flags}){body}") }
}

pub fn literal(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization")
}

pub fn validate(ast: &N2File, target: &str) -> Result<(), CodegenError> {
    super::capabilities::validate(ast, target)?;
    for block in &ast.blocks {
        if let Block::Rule(rule) = block {
            for source in &rule.blacklist {
                let (body, flags) = parts(source);
                let fail = |message: String| CodegenError { target: target.into(), message };
                if body.len() > 4096 || !flags.chars().all(|c| "igm".contains(c)) {
                    return Err(fail("Unsupported or oversized blacklist pattern".into()));
                }
                regex::Regex::new(&expression(source)).map_err(|e| fail(format!("Invalid blacklist regex: {e}")))?;
                // C's portable standard library has no regex engine. Never downgrade a rule to contains.
                if target == "c" && (!flags.is_empty() || body.chars().any(|c| ".^$*+?()[]{}|\\".contains(c))) {
                    return Err(fail("C target supports literal blacklist patterns only; use a regex-capable target".into()));
                }
                if matches!(target, "go" | "python" | "ts" | "typescript" | "cpp")
                    && (body.contains("(?") || body.contains("\\p") || body.contains("\\P") || body.contains("&&") || body.contains("\\u{")) {
                    return Err(fail("Pattern uses syntax outside the portable regex subset".into()));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn release_codegen_regex_flags_and_quotes() {
        assert!(regex::Regex::new(&expression("/secret[0-9]+/i")).unwrap().is_match("SECRET123"));
        assert_eq!(literal("a\"b\\c"), "\"a\\\"b\\\\c\"");
    }
}
