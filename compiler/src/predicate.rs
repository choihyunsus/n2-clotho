//! Bounded Rust regular-expression predicates shared by native callers and the WASM runtime.
use regex::{Regex, RegexBuilder};

const MAX_PATTERN_BYTES: usize = 4096;
const MAX_INPUT_BYTES: usize = 1024 * 1024;
const MAX_AUTOMATON_BYTES: usize = 64 * 1024;

pub struct BoundedPredicate(Regex);

impl BoundedPredicate {
    pub fn new(pattern: &str, flags: &str) -> Result<Self, String> {
        if pattern.len() > MAX_PATTERN_BYTES {
            return Err("Pattern exceeds 4096 UTF-8 bytes".into());
        }
        if flags.len() > 16 || !flags.chars().all(|flag| "igmsu".contains(flag)) {
            return Err("Supported predicate flags are i, g, m, s and u".into());
        }
        RegexBuilder::new(pattern)
            .case_insensitive(flags.contains('i'))
            .multi_line(flags.contains('m'))
            .dot_matches_new_line(flags.contains('s'))
            .size_limit(MAX_AUTOMATON_BYTES)
            .dfa_size_limit(MAX_AUTOMATON_BYTES)
            .build()
            .map(Self)
            .map_err(|_| "Invalid or oversized Rust regular expression".into())
    }

    pub fn test(&self, input: &str) -> Result<bool, String> {
        if input.len() > MAX_INPUT_BYTES {
            return Err("Predicate input exceeds 1 MiB".into());
        }
        Ok(self.0.is_match(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_unicode_classes_and_stateless_flags_are_preserved() {
        let words = BoundedPredicate::new(r"^\w+$", "u").unwrap();
        assert!(words.test("한글_123").unwrap());
        assert!(!words.test("hello world").unwrap());
        let multiline = BoundedPredicate::new("^secret$", "igm").unwrap();
        assert!(multiline.test("before\nSECRET\nafter").unwrap());
        assert!(multiline.test("SECRET").unwrap());
        assert!(multiline.test("SECRET").unwrap());
    }

    #[test]
    fn unsupported_patterns_and_resource_overflows_fail_explicitly() {
        for pattern in [r"(a)\1", "(?=a)", "[", "a{1000}{1000}"] {
            assert!(BoundedPredicate::new(pattern, "").is_err(), "{pattern}");
        }
        assert!(BoundedPredicate::new(&"a".repeat(MAX_PATTERN_BYTES + 1), "").is_err());
        assert!(BoundedPredicate::new("x", "z").is_err());
        assert!(BoundedPredicate::new("x", "").unwrap().test(&"x".repeat(MAX_INPUT_BYTES + 1)).is_err());
        let predicate = BoundedPredicate::new("(a+)+$", "").unwrap();
        assert!(!predicate.test(&("a".repeat(10000) + "!")).unwrap());
    }
}
