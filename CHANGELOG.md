# Changelog

All notable changes to Clotho will be documented in this file.

Format based on [Keep a Changelog](https://keepachangelog.com/).

---

## [3.1.0] — 2026-03-26

### Repositioning: From Security Tool to Universal Rule Compiler

The README and project identity have been completely rewritten to reflect Clotho's true purpose — a **general-purpose compiled instruction language** for AI agents, not just a security tool.

### Changed
- **README full rewrite (EN + KR)**: Repositioned from "security/blocking" to "universal rule compiler"
- **New tagline**: "Markdown rules are dead" → **"Define it. Compile it. Enforce it."**
- **6 use case sections**: Coding Standards, Workflows, Agent Personas, Project Conventions, Security Gates, Multi-Agent Coordination
- **Security repositioned**: Now presented as one use case among many, with [Ark](https://github.com/choihyunsus/n2-ark) as the security layer **built on top of** Clotho contracts
- **Ecosystem diagram**: Clotho shown as the **foundation layer**, with Ark/Soul as applications built on Clotho
- **npm description**: Updated to "Compiled instruction language for AI agents — Define, compile, and enforce rules that markdown can't"
- **4-panel comic**: Regenerated with coding standards enforcement theme (replaces security-focused comic)

### Why This Matters
- Previous README led readers to see Clotho as "just a security blocker"
- Community feedback confirmed the security-first examples caused misunderstanding
- Clotho's real value is compiling **any behavior rule** (coding standards, workflows, personas, conventions) into enforceable contracts — security is just one layer (which Ark handles)

---

## [3.0.0] — 2026-03-25

### Multi-Target Compilation Engine

Major release introducing compiled output for **6 target languages**.

### Added
- **Multi-target codegen**: Compile `.n2` contracts to Rust (.n2rs), C (.n2c), C++ (.n2c2), Go (.n2go), Python (.n2py), TypeScript (.n2ts)
- **BackendRegistry**: Extensible trait-based `CodeGenerator` architecture for adding new backends
- **MCP server**: 5 tools — `clotho_compile`, `clotho_batch`, `clotho_validate`, `clotho_backends`, `clotho_inspect`
- **CLI commands**: `n2c compile <file> --target <TARGET>`, `n2c backends`
- **SQL query engine**: `SELECT * FROM rules WHERE scope = 'command'`
- **Contract simulation**: `n2c simulate` for state machine integrity verification
- **WASM bindings**: `parse_n2_wasm`, `validate_n2_wasm`, `query_n2_wasm` for Node.js

### Fixed (v3.0.0 Patch — 2026-03-25)
- **[CRITICAL] C backend**: Missing `#endif` header guard — generated `.n2c` files would fail `gcc` compilation
- **[CRITICAL] CLI**: `--target rust` (space-separated) silently compiled all targets instead of just rust
- **[CRITICAL] Version**: `Cargo.toml` showed `2.0.0` while CLI reported `3.0.0` — now uses `env!("CARGO_PKG_VERSION")`
- **[SECURITY] Blacklist pattern parsing**: `/DROP TABLE/i` was cleaned to `"DROP TABLE/"` (trailing slash) — security rules were bypassable. Fixed with proper regex `/pattern/flags` parser
- **[BUG] Go backend**: Unconditional `import ("fmt", "strings")` caused Go compiler errors when Contract or Rule blocks were absent. Now scans AST to emit only needed imports
- **[BUG] Timestamp**: `chrono_now()` returned fake string `"compiled-by-n2c-v3.0.0"` instead of real ISO 8601 time

### Improved
- **DRY refactoring**: `collect_states()` and `clean_pattern()` extracted to shared `mod.rs` (was duplicated in all 6 backends)
- **Schema consistency**: All 5 non-Rust backends now document `Block::Schema` as Rust-only with inline comments
- **TypeScript safety**: `emit_workflow()` guards against empty workflow names (prevents `.unwrap()` panic)
- **PowerShell compatibility**: `package.json` build script `&&` → `;`
- **Internationalization**: CLI messages converted from Korean to English for global distribution
- **Test suite**: 17 automated tests verifying compilation output correctness (`npm test`)

---

## [2.0.0] — 2026-03-24

### Added
- PEG grammar parser for `.n2` language (179 rules)
- AST type system with 8 block types (`@meta`, `@import`, `@schema`, `@contract`, `@rule`, `@workflow`, `@query`, `@semantic`)
- Schema validator with error/warning severity levels
- Contract runtime with state machine simulation
- SQL query engine for rule inspection
- CLI tool (`n2c`) with parse, validate, simulate, query commands

---

## [1.0.0] — 2026-03-23

### Added
- Initial `.n2` language specification
- Basic parser and AST definitions
- Single-target Rust codegen proof of concept
