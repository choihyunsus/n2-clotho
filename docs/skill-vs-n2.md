# `.md` Skills vs `.n2` Contracts

> **Why replace markdown-based skills with compiled `.n2` contracts?**

Every AI coding tool uses markdown files to define agent behavior — `GEMINI.md`, `.cursorrules`, `CLAUDE.md`, skill files in `workflows/*.md`. They all share the same fatal flaw: **zero enforcement**.

This document compares the traditional markdown skill approach with Clotho's compiled `.n2` contracts.

---

## The Core Problem

```
 .md Skill: "Please follow these 9 coding pillars"
 → AI: "Got it!" → uses `any` 47 times, skips verification, deploys untested code

 .n2 Contract: @rule TypeSafety { checks: [no_any_type] }
 → AI attempts `any` → BLOCKED before it reaches your codebase
```

**Markdown is a suggestion. `.n2` is compiled law.**

---

## Side-by-Side Comparison

### 1. Workflow Enforcement

#### `.md` Skill (hope-based)

```markdown
## Mandatory Workflow
1. Boot → n2_boot()
2. Pre-code → Call n2_coding()
3. Code → Write production-grade code
4. Post-code → Call n2_coding_verify()
5. End → n2_work_end()
```

> AI reads this, then skips step 2 and 4. Nothing happens.

#### `.n2` Contract (compiler-enforced)

```n2
@contract CodingLifecycle {
 scope: session
 states: CodingState

 transitions {
 COLD -> BOOTED : on n2_boot
 BOOTED -> SKILLED : on n2_coding
 SKILLED -> WORKING : on n2_work_start
 WORKING -> VERIFYING : on n2_coding_verify
 VERIFYING -> DONE : on n2_work_end
 }

 invariant {
 on n2_work_start requires state == SKILLED
 => "n2_coding must be loaded before starting work"

 on n2_work_end requires state == VERIFYING
 => "Cannot end work without running n2_coding_verify"
 }
}
```

> AI tries to skip `n2_coding` → ** BLOCKED**. State machine won't transition.

---

### 2. Coding Standards

#### `.md` Skill

```markdown
## 9 Pillars (Summary)
| # | Pillar | Key Rule |
| 1 | Type Safety | ZERO `as any`, explicit return types |
| 2 | Complexity | Function < 50 lines, CC < 10, file < 500 lines |
| 6 | Conventions | ESLint strict, naming rules |
```

> AI "agrees" then writes 800-line files with `as any` everywhere.

#### `.n2` Contract

```n2
@rule TypeSafety {
 description: "Zero tolerance for type violations"
 scope: code
 enforce: strict

 checks: [
 no_any_type,
 no_ts_ignore,
 no_ts_expect_error,
 explicit_return_types
 ]
}

@rule Complexity {
 description: "Keep code simple and readable"
 scope: code
 enforce: strict

 checks: [
 max_file_lines: 500,
 max_function_lines: 50,
 cyclomatic_complexity: 10
 ]
}

@rule NamingConventions {
 scope: code
 enforce: strict

 checks: [
 components: /^[A-Z][a-zA-Z]+$/,
 variables: /^[a-z][a-zA-Z0-9]*$/,
 constants: /^[A-Z_]+$/
 ]
}
```

> Each rule is individually checkable, queryable, and enforced.

---

### 3. Post-Coding Verification

#### `.md` Skill

```markdown
## Post-Coding Checklist
- [ ] All test/scratch/temp files deleted
- [ ] No legacy code left
- [ ] No unused imports
- [ ] No console.log debug statements
- [ ] ZERO `as any`
- [ ] Every function < 50 lines
- [ ] `tsc --noEmit` = 0 errors
- [ ] Would you be proud to deploy this today?
```

> Checklist exists. AI ignores half of it.

#### `.n2` Contract

```n2
@workflow PostCodingVerify {
 description: "Automated post-coding verification"
 trigger: on_event(n2_coding_verify)
 enforce: strict

 step check_temp_files {
 action: scan_for(patterns: [/\.tmp$/, /\.bak$/, /scratch/])
 expect { none_found => continue }
 }

 step check_unused_imports {
 action: lint_check(rule: "no-unused-imports")
 expect { pass => continue }
 }

 step check_console_log {
 action: scan_for(patterns: [/console\.log/])
 expect { none_found => continue }
 }

 step type_check {
 action: run_command("tsc --noEmit")
 expect {
 exit_code == 0 => continue
 fail => abort with "TypeScript errors must be fixed before completion"
 }
 }

 step senior_review {
 depends_on: [check_temp_files, check_unused_imports, check_console_log, type_check]
 action: self_review(question: "Would you be proud to deploy this today?")
 required: true
 }
}
```

> Every step must pass. Failure blocks completion. No shortcuts.

---

### 4. Configuration

#### `.md` Skill

```markdown
## Rules
- Max file lines: 500
- Max function lines: 50
- Use TypeScript strict mode
```

> Hardcoded values buried in prose. No way to override per-project.

#### `.n2` Contract

```n2
@schema CodingConfig {
 max_file_lines: int [default: 500]
 max_function_lines: int [default: 50]
 max_complexity: int [default: 10]
 strict_mode: bool [default: true]
 allowed_languages: string[] [default: ["typescript", "python", "rust"]]
}
```

> Type-checked, defaulted, overridable. Import and customize per-project:

```n2
@import { CodingConfig } from "n2-coding.n2"

# Override for this specific project
@meta {
 config: CodingConfig {
 max_file_lines: 300 # Stricter for this project
 allowed_languages: ["typescript"]
 }
}
```

---

### 5. Querying & Debugging

#### `.md` Skill

```
How to find which rules apply? → Ctrl+F through the document
How to check if rules conflict? → Read everything manually
How to audit what's enforced? → You can't
```

#### `.n2` Contract

```bash
# List all strict rules
$ n2c query coding.n2 "SELECT name, scope FROM rules WHERE enforce = 'strict'"

┌──────────────────────┬───────┬─────────┐
│ name │ scope │ enforce │
├──────────────────────┼───────┼─────────┤
│ TypeSafety │ code │ strict │
│ Complexity │ code │ strict │
│ NamingConventions │ code │ strict │
│ Security │ code │ strict │
└──────────────────────┴───────┴─────────┘

# Validate all contracts for integrity
$ n2c validate coding.n2

 State machine: CodingLifecycle — 5 transitions, integrity verified
 All rules: 4 rules, 0 errors, 0 warnings
 Workflow: PostCodingVerify — 5 steps, dependency chain valid
```

---

## Summary

| Aspect | `.md` Skill | `.n2` Contract |
|--------|------------|----------------|
| **Format** | Free-form prose | Structured blocks |
| **Parsing** | AI "reads" it | PEG grammar → AST |
| **Enforcement** | Hope | Compiler-blocked |
| **Workflow order** | Numbered list | State machine transitions |
| **Configuration** | Hardcoded in text | Schema with types + defaults |
| **Querying** | Ctrl+F | SQL queries |
| **Debugging** | Read everything | `n2c validate` pinpoints errors |
| **Cross-project** | Copy-paste | `@import` from shared files |
| **Versioning** | Manual | `@meta { version: "1.0.0" }` |
| **Determinism** | Different every run | Same input → same plan |

---

## Real-World Impact

In our own N2 ecosystem, switching from `.md` skills to `.n2` contracts eliminated:

- AI skipping boot sequence → **state machine blocks it**
- AI using `as any` → **TypeSafety rule blocks it**
- AI ending work without verification → **CodingLifecycle invariant blocks it**
- AI running destructive commands → **Ark security gate blocks it**
- Multiple agents editing same file → **ownership contract blocks it**

> *"The difference between `.md` and `.n2` is the difference between asking nicely and having a compiler."*

---

 [Get Clotho](https://www.npmjs.com/package/n2-clotho) · [Back to README](../README.md) · [Ark Security Layer](https://github.com/choihyunsus/n2-ark)
