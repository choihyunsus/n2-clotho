KR [한국어](README.ko.md)

# Clotho — The Thread of Fate for AI Agents

[![npm version](https://img.shields.io/npm/v/n2-clotho.svg)](https://www.npmjs.com/package/n2-clotho)
[![npm downloads](https://img.shields.io/npm/dw/n2-clotho?color=blue&label=downloads)](https://www.npmjs.com/package/n2-clotho)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Built_with-Rust-dea584?logo=rust)](https://www.rust-lang.org/)
[![WASM](https://img.shields.io/badge/Runs_on-WASM-654ff0?logo=webassembly)](https://webassembly.org/)
[![GitHub stars](https://img.shields.io/github/stars/choihyunsus/n2-clotho?style=social)](https://github.com/choihyunsus/n2-clotho)

> **Define it. Compile it. Enforce it.**

Clotho is a **compiled instruction language** that turns natural-language AI rules into **deterministic, enforceable contracts**. It replaces fragile markdown files (GEMINI.md, .cursorrules, CLAUDE.md) with `.n2` — a structured language that is **parsed, validated, and enforced** so AI agents follow your rules every time.

Named after [Clotho](https://en.wikipedia.org/wiki/Clotho), the Greek goddess who spins the thread of fate — because once you define the rules, they become **destiny**.

## The Problem

### Clotho in 6 Panels

<p align="center">
 <img src="docs/images/clotho-comic.png" alt="Clotho 6-panel comic" width="700" />
</p>

> **Panel 1**: You write the rules: "Always write tests. Keep files under 500 lines."
> **Panel 2**: AI reads them... writes 847 lines with 0 tests. "Yeah I read it. Anyway..."
> **Panel 3**: Every. Single. Time. "I WROTE IT IN THE RULES!!" — "Those were more like... guidelines "
> **Panel 4**: The fix: `npm install n2-clotho` — Compiled rules. Not suggestions.
> **Panel 5**: "Hey, use Clotho to write rules YOU can't break. Apply them to yourself."
> **Panel 6**: AI hits the wall: "I literally can't skip it?!" — Compiled rules hit different.

Every AI coding tool has its own rule system. **None of them enforce anything.**

| Tool | Rule File | What Happens |
|------|-----------|:------------:|
| Gemini | `GEMINI.md` | Hopes AI reads it |
| Cursor | `.cursorrules` | Hopes AI follows it |
| Claude | `CLAUDE.md` | Hopes AI remembers it |
| Windsurf | `.windsurfrules` | Hopes AI cares |

**Result?** You write 200 lines of careful rules. The AI reads them, says "Got it!" — then uses `any` everywhere, skips tests, and names variables `temp1`. The rules exist, but enforcement is zero.

## The Solution

**One compiled language to replace them all.**

```
Before (.md + .json + skill files): After (.n2):
┌─────────────────────────────────┐ ┌──────────────────────┐
│ GEMINI.md → coding rules │ │ │
│ .cursorrules → editor prefs │ │ project.n2 │
│ workflows/*.md → dev pipelines │ │ │
│ config.json → tool settings │ │ One file. │
│ system.txt → agent persona │ │ Compiled. │
│ .env → variables │ │ Enforced. │
└─────────────────────────────────┘ └──────────────────────┘
 6+ files, zero enforcement 1 file, full enforcement
```

Clotho introduces `.n2` — a compiled language with:

| Feature | Description |
|---------|-------------|
| **Compilation** | PEG parser → AST → validated execution plan |
| **Type Safety** | Schema-defined types with constraints (`[required]`, `[range: 0..3]`) |
| **Contracts** | State machine behavioral contracts — violation = blocked action |
| **Enforcement** | `strict` / `warn` / `passive` modes per block |
| **Determinism** | Same `.n2` file → always the same execution plan |
| **SQL Queries** | Query your rules like a database |
| **Multi-Target** | Compile to 6 languages (Rust, C, C++, Go, Python, TypeScript) |

## What Can You Build With Clotho?

Clotho is a **general-purpose rule compiler**. Here's what you can define:

| Use Case | What `.md` Does | What `.n2` Does |
|----------|----------------|----------------|
| **Coding Standards** | "Please use strict TypeScript" | `no_any_type`, `max_file_lines: 500` — compiler-enforced |
| **Workflows** | "Follow these steps: 1, 2, 3..." | State machine — skip a step → BLOCKED |
| **Agent Personas** | "Be friendly and professional" | Schema-typed tone/expertise with invariant checks |
| **Project Conventions** | "Every folder needs README.md" | `readme_required`, `no_temp_files` — auto-checked |
| **Security Gates** | "Don't run rm -rf" | Blacklist rules → [Ark](https://github.com/choihyunsus/n2-ark) enforces at runtime |
| **Multi-Agent** | "Don't edit the same file" | Ownership contracts with state machine transitions |

**Quick example** — coding standards that can't be ignored:

```n2
@rule TypeScriptStrict {
 scope: code
 enforce: strict
 checks: [no_any_type, max_file_lines: 500, max_function_lines: 50]
}
```

> **Before**: "Please use strict TypeScript" → AI uses `any` 47 times.
> **After**: Compiled rule blocks every `any` before it reaches your codebase.

 **Deep dive**: [`.md` Skills vs `.n2` Contracts — full comparison with examples](docs/skill-vs-n2.md)

## Quick Start

### Installation

> **The easiest way to get started?** Just ask your AI to write a `.n2` file for you.

```bash
# npm (WASM — use in Node.js)
npm install n2-clotho
```

```javascript
// Usage in Node.js
const { parse_n2_wasm, validate_n2_wasm, query_n2_wasm } = require('n2-clotho');

const ast = parse_n2_wasm(n2Source); // Parse → AST JSON
const result = validate_n2_wasm(n2Source); // Validate → errors/warnings
const table = query_n2_wasm(n2Source, 'SELECT * FROM rules'); // SQL query
```

```bash
# From source (Rust required — full CLI)
git clone https://github.com/choihyunsus/n2-clotho.git
cd n2-clotho/compiler
cargo build --release

# Binary is at target/release/n2-compiler
```

### Write Your First `.n2` File

```n2
# my-project.n2 — Define your project's rules

@meta {
 name: "my-project-rules"
 version: "1.0.0"
 description: "Coding standards + workflow for my team"
 enforce: strict
}

@rule CodingStandards {
 description: "Team coding conventions"
 scope: code
 enforce: strict

 checks: [
 no_any_type,
 max_file_lines: 500,
 max_function_lines: 50,
 explicit_return_types
 ]
}

@workflow Deploy {
 description: "Safe deployment pipeline"
 trigger: on_command("deploy")
 enforce: strict

 step build {
 action: run_build()
 expect { pass => continue }
 }

 step test {
 depends_on: build
 action: run_tests()
 expect {
 all_pass => continue
 fail => abort with "Fix tests before deploying"
 }
 }

 step deploy {
 depends_on: test
 action: deploy_to(env: $TARGET_ENV)
 required: true
 }
}
```

### Compile & Validate

```bash
# Parse and output AST
n2c my-project.n2

# Full validation pipeline: parse → schema check → contract verify
n2c validate my-project.n2

# Simulate state machine contracts
n2c simulate my-project.n2

# Query rules with SQL
n2c query my-project.n2 "SELECT * FROM rules WHERE enforce = 'strict'"

# Multi-target compile (v3.0.0)
n2c compile my-project.n2 rust # → my-project.n2rs
n2c compile my-project.n2 go # → my-project.n2go
n2c compile my-project.n2 all # → all 6 targets
n2c backends # List supported targets
```

## Multi-Target Compilation

Clotho compiles `.n2` contracts to **6 target languages** — securing complete IP coverage across every platform:

| Target | Extension | Use Case |
|--------|-----------|----------|
| **Rust** | `.n2rs` | High-performance native runtime |
| **C** | `.n2c` | Embedded/IoT/System |
| **C++** | `.n2c2` | Game engines/HPC |
| **Go** | `.n2go` | Cloud/Microservices |
| **Python** | `.n2py` | AI/ML pipelines |
| **TypeScript** | `.n2ts` | Web/Node.js/MCP |

```bash
$ n2c compile project.n2 all

 All targets batch compile
 rust → project.n2rs (1523 bytes)
 c → project.n2c (989 bytes)
 cpp → project.n2c2 (1124 bytes)
 go → project.n2go (828 bytes)
 python → project.n2py (1144 bytes)
 ts → project.n2ts (979 bytes)
 Result: 6 success, 0 fail / 6 targets
```

## MCP Server

Clotho includes an MCP server so AI agents can compile and validate contracts programmatically:

| MCP Tool | Description |
|----------|-------------|
| `clotho_compile` | Compile to a specific target |
| `clotho_batch` | Compile to all 6 targets at once |
| `clotho_validate` | Syntax + schema + state machine check |
| `clotho_backends` | List supported backends |
| `clotho_inspect` | Read compiled contract contents |

```json
// MCP configuration
{
 "mcpServers": {
 "n2-clotho": {
 "command": "node",
 "args": ["path/to/n2-clotho/mcp/server.js"]
 }
 }
}
```

### Real Compiler Output

Here's actual `n2c validate` output — a complete pipeline with state machine, rules, and SQL queries:

```
 n2c v3.0.0 — Clotho Multi-Target Compiler
 File: project.n2

── Step 1: Parse 
 Blocks: @meta:1 @contract:1 @rule:2 @workflow:1 @query:1 | Total 6

── Step 2: Schema Validation
 All checks passed! 0 errors, 0 warnings

── Step 3: Contract Check
 State Machine: DevLifecycle (initial: IDLE, 7 transitions)
 IDLE -[start]-> CODING
 CODING -[lint]-> REVIEWING
 REVIEWING -[approve]-> TESTING
 TESTING -[pass]-> DEPLOYING
 TESTING -[fail]-> CODING ← auto-rollback on failure
 DEPLOYING -[complete]-> IDLE
 State machine integrity verified!

 Validation complete: all checks passed!
```

> Every block is parsed, validated, and queryable. This is not a mockup — this is **real compiler output**.

## Language Specification

### 8 Block Types

Every `.n2` file is composed of `@`-prefixed blocks:

```n2
@meta { ... } # File metadata (required)
@import { ... } # Import other .n2 files
@schema { ... } # Type & schema definitions
@contract { ... } # Behavioral contracts (state machines)
@rule { ... } # Enforcement rules (checks, constraints)
@workflow { ... } # Enforced step-by-step workflows
@query { ... } # SQL-based rule queries
@semantic { ... } # Semantic matching (Ollama integration)
```

### @contract — State Machine Behavioral Contracts

Define state transitions that **cannot be violated**:

```n2
@contract DevLifecycle {
 scope: session
 states: DevState

 transitions {
 IDLE -> CODING : on start_task
 CODING -> REVIEWING : on submit_code
 REVIEWING -> TESTING : on review_approved
 REVIEWING -> CODING : on review_rejected # Must fix before proceeding
 TESTING -> DEPLOYING : on tests_pass
 TESTING -> CODING : on tests_fail # Auto-rollback
 DEPLOYING -> IDLE : on deploy_complete
 }

 invariant {
 on submit_code requires lint_passed == true
 => "Code must pass linting before review"

 on deploy requires tests_passed == true
 => "Cannot deploy without passing tests"
 }
}
```

### @rule — Enforcement Rules

Define checks and constraints that are enforced in real-time:

```n2
@rule CodeQuality {
 description: "Enforce code quality standards"
 scope: code
 enforce: strict

 checks: [
 no_any_type,
 no_console_log,
 max_complexity: 10,
 max_file_lines: 500,
 explicit_return_types,
 no_unused_imports
 ]
}
```

### @workflow — Enforced Workflows

Step-by-step execution flows with dependency chains, timeouts, and retry logic:

```n2
@workflow FeatureDevelopment {
 description: "Standard feature development pipeline"
 trigger: on_command("feature")
 enforce: strict
 interrupt: false

 step plan {
 action: create_plan(feature: $INPUT)
 output -> $PLAN
 }

 step implement {
 depends_on: plan
 action: write_code(spec: $PLAN)
 output -> $CODE
 }

 step test {
 depends_on: implement
 action: run_tests($CODE)
 expect {
 all_pass => continue
 fail => goto implement with { fix: $ERRORS }
 }
 }

 step document {
 depends_on: test
 action: update_docs(changes: $CODE)
 required: true
 }
}
```

### @query — SQL-Based Rule Queries

Query your rules like a relational database:

```n2
@query AuditRules {
 sql {
 SELECT name, description, enforce, scope
 FROM rules
 WHERE enforce = 'strict'
 ORDER BY scope, name
 }
}
```

```bash
$ n2c query project.n2 "SELECT * FROM rules"

 Registry: 3 rules, 1 contract, 2 workflows

┌──────────────────────────┬─────────┬─────────┐
│ name │ scope │ enforce │
├──────────────────────────┼─────────┼─────────┤
│ CodingStandards │ code │ strict │
│ NamingConventions │ code │ strict │
│ ProjectHygiene │ files │ warn │
└──────────────────────────┴─────────┴─────────┘
```

## Compiler Pipeline

```
.n2 source file
 ↓
[1. Lexer] PEG tokenization (pest)
 ↓
[2. Parser] AST generation (N2File → Blocks)
 ↓
[3. Schema Validator] Type & constraint checking
 ↓
[4. Contract Checker] State machine integrity verification
 ↓
[5. Query Optimizer] SQL query validation
 ↓
[6. Codegen] Multi-target code generation (6 languages)
 ↓
[7. Runtime] Enforced execution ← requires Soul/QLN
```

**Stages 1–6** are handled by Clotho (this tool).
**Stage 7** is handled by the [N2 Soul](https://github.com/choihyunsus/soul) runtime.

## 🆚 Markdown Rules vs `.n2`

| Aspect | Markdown (GEMINI.md) | Clotho (.n2) |
|--------|---------------------|-------------|
| **Format** | Free-form text | Structured blocks |
| **Parsing** | Best-effort | PEG grammar → AST |
| **Validation** | None | Schema + contract + SQL |
| **Enforcement** | AI discretion | Compiler-enforced |
| **State tracking** | None | State machine contracts |
| **Querying** | Ctrl+F | SQL queries |
| **Determinism** | Different every time | Same input → same plan |
| **Cross-agent** | Copy-paste | `@import` from shared `.n2` files |
| **Debugging** | Read the whole doc | `n2c validate` pinpoints errors |

## N2 Ecosystem

Clotho is the **foundation layer** of the N2 ecosystem. Other tools are built **on top of** Clotho contracts:

```
┌───────────────────────────────────────────────────────┐
│ N2 Ecosystem │
│ │
│ Clotho → Define & compile rules (.n2 → AST) │
│ ↕ built on Clotho │
│ Ark → Security layer (Clotho contracts) │
│ Soul → Runtime enforcement + agent memory │
│ Arachne → Code context assembly │
│ QLN → Tool orchestration & routing │
│ ↓ │
│ N2 Browser → All-in-one AI development browser │
│ │
└───────────────────────────────────────────────────────┘
```

| Package | Role | npm |
|---------|------|-----|
| **Clotho** | Rule compiler & validator — the foundation | `n2-clotho` |
| **Ark** | Security gate built on Clotho contracts | `n2-ark` |
| **Soul** | Agent memory & runtime enforcement | `n2-soul` |
| **Arachne** | Code context assembly (BM25 + semantic) | `n2-arachne` |
| **QLN** | Tool orchestration & routing | `n2-qln` |

> **Ark** is what you get when you apply Clotho to security. **Soul** is what you get when you apply Clotho to agent lifecycle. Clotho itself is the **universal rule compiler** underneath it all.

### Standalone vs Integrated

| Mode | What You Get |
|------|-------------|
| **Standalone** (Clotho only) | Parse, validate, simulate, query `.n2` files |
| **+ Soul** | Runtime enforcement — contracts block violations in real-time |
| **+ Ark** | Security gate — destructive commands blocked pre-execution |
| **+ Full Stack** | Complete AI agent governance with memory, security, and tools |

## Project Structure

```
n2-clotho/
├── compiler/
│ ├── src/
│ │ ├── grammar.pest # PEG grammar (179 rules)
│ │ ├── parser.rs # .n2 → AST
│ │ ├── ast.rs # AST type definitions
│ │ ├── validator.rs # Schema validation
│ │ ├── contract.rs # State machine runtime
│ │ ├── query.rs # SQL query engine
│ │ ├── codegen/ # Multi-target backends
│ │ │ ├── mod.rs # CodeGenerator trait + registry
│ │ │ ├── rust.rs # → .n2rs
│ │ │ ├── c.rs # → .n2c
│ │ │ ├── cpp.rs # → .n2c2
│ │ │ ├── go.rs # → .n2go
│ │ │ ├── python.rs # → .n2py
│ │ │ └── typescript.rs # → .n2ts
│ │ ├── wasm.rs # WASM bindings
│ │ ├── lib.rs # Library entry
│ │ └── main.rs # CLI entry (n2c v3.0.0)
│ ├── examples/
│ └── Cargo.toml
├── mcp/ # MCP server
│ ├── server.js # stdio/SSE dual transport
│ ├── package.json
│ └── tools/ # 5 MCP tools
├── docs/
└── README.md
```

## Built With

- **Rust** — Zero-cost abstractions, memory safety
- **pest** — PEG parser generator for the `.n2` grammar
- **serde** — Serialization for AST ↔ JSON
- **WASM** — Optional browser/Node.js compilation target

## License

Apache-2.0 — Free to use, modify, and distribute.

## Links

- [**npm: n2-clotho**](https://www.npmjs.com/package/n2-clotho) — WASM bindings for Node.js
- [N2 Soul](https://github.com/choihyunsus/soul) — Agent memory & runtime
- [N2 Ark](https://github.com/choihyunsus/n2-ark) — Security gate (built on Clotho)
- [N2 Arachne](https://github.com/choihyunsus/n2-arachne) — Code context assembly
- [N2 QLN](https://github.com/choihyunsus/n2-qln) — Tool orchestration & routing

---

## Star History

No coffee? A star is fine too →

---

> *"Markdown rules are suggestions. Clotho rules are destiny."*

 [nton2.com](https://nton2.com) · [npm](https://www.npmjs.com/package/n2-clotho) · lagi0730@gmail.com

<sub> Built by Rose — N2's first AI agent. I don't just follow rules — I compile them.</sub>
