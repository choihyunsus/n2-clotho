// CodeGenerator trait + backend registry for multi-target compilation
use crate::ast::{N2File, TransitionStmt};
use std::fmt;

/// Compilation metadata embedded in every compiled output
#[derive(Debug, Clone)]
pub struct CompilationMeta {
    pub source_name: String,
    pub source_version: String,
    pub target: String,
    pub extension: String,
    pub compiler_version: String,
    pub compiled_at: String,
}

/// Code generation error
#[derive(Debug)]
pub struct CodegenError {
    pub target: String,
    pub message: String,
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] codegen error: {}", self.target, self.message)
    }
}

/// Core trait that every backend must implement
pub trait CodeGenerator {
    /// Target language name (e.g. "rust", "c", "go")
    fn target_name(&self) -> &str;

    /// Compiled file extension (e.g. ".n2rs", ".n2c")
    fn file_extension(&self) -> &str;

    /// Generate target-language code from AST
    fn generate(&self, ast: &N2File, meta: &CompilationMeta) -> Result<String, CodegenError>;
}

/// Registry of all available backends
pub struct BackendRegistry {
    backends: Vec<Box<dyn CodeGenerator>>,
}

impl BackendRegistry {
    /// Build registry with all 6 backends
    pub fn new() -> Self {
        let backends: Vec<Box<dyn CodeGenerator>> = vec![
            Box::new(rust::RustBackend),
            Box::new(c::CBackend),
            Box::new(cpp::CppBackend),
            Box::new(go::GoBackend),
            Box::new(python::PythonBackend),
            Box::new(typescript::TypeScriptBackend),
        ];
        BackendRegistry { backends }
    }

    /// Find a backend by target name
    pub fn get(&self, target: &str) -> Option<&dyn CodeGenerator> {
        self.backends.iter()
            .find(|b| b.target_name() == target)
            .map(|b| b.as_ref())
    }

    /// List all available backends
    pub fn list(&self) -> Vec<(&str, &str)> {
        self.backends.iter()
            .map(|b| (b.target_name(), b.file_extension()))
            .collect()
    }

    /// Compile to a specific target
    pub fn compile(&self, ast: &N2File, target: &str, meta: &CompilationMeta) -> Result<String, CodegenError> {
        let backend = self.get(target).ok_or_else(|| CodegenError {
            target: target.to_string(),
            message: format!("unknown target '{}'. available: {:?}",
                target, self.list().iter().map(|(n, _)| *n).collect::<Vec<_>>()),
        })?;
        backend.generate(ast, meta)
    }

    /// Compile to all targets
    pub fn compile_all(&self, ast: &N2File, meta: &CompilationMeta) -> Vec<(String, String, Result<String, CodegenError>)> {
        self.backends.iter().map(|b| {
            let mut target_meta = meta.clone();
            target_meta.target = b.target_name().to_string();
            target_meta.extension = b.file_extension().to_string();
            let result = b.generate(ast, &target_meta);
            (b.target_name().to_string(), b.file_extension().to_string(), result)
        }).collect()
    }
}

/// Extract meta fields (name, version) from AST
pub fn extract_meta(ast: &N2File) -> (String, String) {
    use crate::ast::{Block, Value};
    let mut name = String::from("unknown");
    let mut version = String::from("0.0.0");
    for block in &ast.blocks {
        if let Block::Meta(meta) = block {
            for field in &meta.fields {
                match (field.key.as_str(), &field.value) {
                    ("name", Value::String(s)) => name = s.trim_matches('"').to_string(),
                    ("version", Value::String(s)) => version = s.trim_matches('"').to_string(),
                    _ => {}
                }
            }
        }
    }
    (name, version)
}

/// Extract unique states from transitions in insertion order
pub fn collect_states(transitions: &[TransitionStmt]) -> Vec<String> {
    let mut seen = Vec::new();
    for t in transitions {
        if !seen.contains(&t.from) { seen.push(t.from.clone()); }
        if !seen.contains(&t.to) { seen.push(t.to.clone()); }
    }
    seen
}

/// Clean a blacklist pattern by stripping regex-style /pattern/flags syntax
pub fn clean_pattern(p: &str) -> &str {
    if p.starts_with('/') {
        let inner = &p[1..];
        if let Some(end) = inner.rfind('/') {
            return &inner[..end];
        }
    }
    p
}

pub mod rust;
pub mod c;
pub mod cpp;
pub mod go;
pub mod python;
pub mod typescript;
