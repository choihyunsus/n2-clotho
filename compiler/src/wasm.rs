// WASM bindings — n2c compiler exposed to JavaScript via wasm-bindgen
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::parser::parse_n2;
use crate::validator;
use crate::contract::ContractRuntime;
use crate::query::N2Registry;
use crate::codegen::{BackendRegistry, CompilationMeta, extract_meta};

/// Generate the requested language using the same validated backends as the CLI.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn compile_n2_wasm(source: &str, target: &str, compiled_at: &str) -> Result<String, JsValue> {
    let ast = parse_n2(source).map_err(|error| JsValue::from_str(&error))?;
    let registry = BackendRegistry::new();
    let backend = registry.get(target).ok_or_else(|| JsValue::from_str("Unknown target"))?;
    let (source_name, source_version) = extract_meta(&ast);
    let meta = CompilationMeta {
        source_name, source_version, target: target.into(),
        extension: backend.file_extension().into(),
        compiler_version: format!("n2c v{}", env!("CARGO_PKG_VERSION")),
        compiled_at: compiled_at.into(),
    };
    registry.compile(&ast, target, &meta).map_err(|error| JsValue::from_str(&error.to_string()))
}

/// Parse .n2 source and return AST as JSON string
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_n2_wasm(source: &str) -> Result<String, JsValue> {
    let ast = parse_n2(source)
        .map_err(|e| JsValue::from_str(&e))?;
    serde_json::to_string_pretty(&ast)
        .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
}

/// Validate .n2 source — returns JSON with errors/warnings
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate_n2_wasm(source: &str) -> Result<String, JsValue> {
    let ast = parse_n2(source)
        .map_err(|e| JsValue::from_str(&e))?;

    let errors = validator::validate(&ast);
    let error_count = errors.iter()
        .filter(|e| e.severity == validator::Severity::Error)
        .count();
    let warn_count = errors.iter()
        .filter(|e| e.severity == validator::Severity::Warning)
        .count();

    let error_list: Vec<serde_json::Value> = errors.iter().map(|e| {
        let severity = match e.severity {
            validator::Severity::Error => "Error",
            validator::Severity::Warning => "Warning",
        };
        serde_json::json!({
            "severity": severity,
            "message": e.message,
            "block": e.block,
            "field": e.field,
        })
    }).collect();

    let runtime = ContractRuntime::from_file(&ast);
    let integrity = runtime.check_integrity();
    let integrity_strs: Vec<String> = integrity.iter().map(|v| v.to_string()).collect();

    let result = serde_json::json!({
        "errors": error_list,
        "errorCount": error_count,
        "warningCount": warn_count,
        "machines": runtime.machines.len(),
        "integrityViolations": integrity_strs,
    });

    serde_json::to_string_pretty(&result)
        .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
}

/// Query .n2 source with SQL — returns formatted table string
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn query_n2_wasm(source: &str, sql: &str) -> Result<String, JsValue> {
    let ast = parse_n2(source)
        .map_err(|e| JsValue::from_str(&e))?;
    let registry = N2Registry::from_file(&ast);
    let result = registry.execute_query(sql)
        .map_err(|e| JsValue::from_str(&e))?;
    Ok(result.to_string())
}

/// Extract blacklist patterns from .n2 source — returns JSON array
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn extract_blacklist_wasm(source: &str) -> Result<String, JsValue> {
    let ast = parse_n2(source)
        .map_err(|e| JsValue::from_str(&e))?;
    let mut patterns: Vec<String> = Vec::new();

    for block in &ast.blocks {
        if let crate::ast::Block::Rule(rule) = block {
            for bl in &rule.blacklist {
                patterns.push(bl.clone());
            }
        }
    }

    serde_json::to_string(&patterns)
        .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
}

/// Get compiler version info
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn n2c_version() -> String {
    format!("n2c v{}-wasm (Clotho compiler)", env!("CARGO_PKG_VERSION"))
}
