# .n2 언어 + N2 Runtime 매뉴얼

## 1. .n2 언어란?

AI 에이전트 행동을 **시스템 수준에서 강제**하는 선언적 규칙 언어.
마크다운은 "읽어라~" → .n2는 **"안 하면 못 넘어간다"**.

## 2. 파일 구조

```
md_project/compiler/        ← Rust 컴파일러 (n2c)
  src/grammar.pest           ← PEG 문법 정의
  src/parser.rs              ← 파서 → AST
  src/validator.rs           ← 스키마 검증
  src/contract.rs            ← 상태머신 시뮬레이션
  src/query.rs               ← SQL 쿼리 엔진
  examples/soul-full-rules.n2 ← 실전 예제

n2-browser/soul/rules/      ← 실전 규칙 파일
  agent-constitution.n2      ← 에이전트 통신 계약
  qln-security.n2            ← 보안 블랙리스트 (36패턴)
  soul-config-schema.n2      ← 설정 타입 검증
  soul-boot-workflow.n2      ← 부팅/종료 워크플로우

n2-browser/soul/lib/
  n2-runtime.js              ← JS 런타임 브릿지 (실시간 강제)
```

## 3. .n2 블록 타입

| 블록 | 용도 | 예시 |
|------|------|------|
| `@meta` | 파일 메타데이터 | `name, version, enforce` |
| `@rule` | 블랙리스트/체크 규칙 | `blacklist: [/npm install/]` |
| `@contract` | 상태머신 계약 | `IDLE -> WORKING : on start` |
| `@workflow` | 순차 워크플로우 | `step boot { action: n2_boot() }` |
| `@schema` | 타입 스키마 | `enabled: bool [required]` |

## 4. CLI 명령어

```bash
# 문법 검증 + 상태머신 무결성
n2c validate soul-boot-workflow.n2

# SQL 쿼리
n2c query file.n2 "SELECT * FROM rules"

# 상태머신 시뮬레이션
n2c simulate file.n2
```

## 5. 런타임 동작

```
n2_boot() → .n2 로드 → 상태머신 4개 활성 + 블랙리스트 36패턴
  BootSequence:      COLD → BOOTING → CODING_LOADED → READY
  NeuralChatProtocol: IDLE → SENDING → WAITING → RESPONDING → IDLE
  QLNSiteProtocol:   IDLE → NAVIGATED → READ → ACTING → IDLE
  CodingWorkflow:    IDLE → CODING_READY → WRITING → VERIFYING → IDLE

도구 호출 시 블랙리스트 체크:
  n2_qln_call(tool, args) → checkBlacklist() → 매칭 시 차단!

감사 로그: soul/data/audit/YYYY-MM-DD.log (7일 자동 삭제)
```

## 6. 규칙 추가 방법

1. `soul/rules/` 에 `.n2` 파일 생성
2. `n2c validate` 로 검증
3. MCP 재로드 → 자동 적용

## 7. 향후 로드맵

- [x] n2c → WASM 빌드 (JS에서 Rust 파서 직접 사용) ✅ v0.2.0
- [x] Hot Reload (MCP 재시작 없이 .n2 변경 자동 반영) ✅ watchRules()
- [ ] LSP (VS Code .n2 자동완성)
- [ ] 에이전트 간 계약 공유
