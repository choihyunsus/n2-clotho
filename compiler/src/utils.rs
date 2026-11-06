// Keyword boundaries for advisory terminal-state naming.
pub fn contains_keyword_bounded(text: &str, keyword: &str) -> bool {
    if keyword.is_empty() {
        return false;
    }
    if !keyword.is_ascii() {
        return text.contains(keyword);
    }
    let bytes = text.as_bytes();
    let klen = keyword.len();
    let mut start = 0usize;
    while let Some(rel) = text[start..].find(keyword) {
        let abs = start + rel;
        let end = abs + klen;
        let before_ok = abs == 0 || !bytes[abs - 1].is_ascii_alphanumeric();
        let after_ok = end == bytes.len() || !bytes[end].is_ascii_alphanumeric();
        if before_ok && after_ok {
            return true;
        }
        start = abs + 1;
    }
    false
}
