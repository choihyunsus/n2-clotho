EN [English](README.md)

# Clotho — AI 에이전트의 운명의 실

[![npm version](https://img.shields.io/npm/v/n2-clotho.svg)](https://www.npmjs.com/package/n2-clotho)
[![npm downloads](https://img.shields.io/npm/dw/n2-clotho?color=blue&label=downloads)](https://www.npmjs.com/package/n2-clotho)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Built_with-Rust-dea584?logo=rust)](https://www.rust-lang.org/)
[![WASM](https://img.shields.io/badge/Runs_on-WASM-654ff0?logo=webassembly)](https://webassembly.org/)
[![GitHub stars](https://img.shields.io/github/stars/choihyunsus/n2-clotho?style=social)](https://github.com/choihyunsus/n2-clotho)

> **정의하고. 컴파일하고. 강제한다.**

Clotho는 자연어로 작성된 AI 규칙을 **결정론적이고 강제 실행 가능한 계약**으로 바꾸는 **컴파일형 명령어 언어**입니다. 깨지기 쉬운 마크다운 규칙 파일(GEMINI.md, .cursorrules, CLAUDE.md)을 `.n2`로 대체합니다 — **파싱되고, 검증되고, 강제 실행**되어 AI 에이전트가 매번 규칙을 따르게 합니다.

[클로토(Clotho)](https://ko.wikipedia.org/wiki/%ED%81%B4%EB%A1%9C%ED%86%A0)는 운명의 실을 잣는 그리스 여신입니다 — 규칙을 한 번 정의하면, 그것이 곧 **운명**이 되니까요.

## 문제

### Clotho를 4컷 만화로

<p align="center">
 <img src="docs/images/clotho-comic.png" alt="Clotho 4컷 만화" width="700" />
</p>

> **1화**: AI가 코딩 표준을 읽는다... **2화**: ...절반은 무시한다.
> **3화**: Clotho가 규칙을 강제 가능한 법으로 컴파일. **4화**: 규칙은 운명. 예외 없음.

모든 AI 코딩 도구에는 각자의 규칙 시스템이 있습니다. **그 어느 것도 강제하지 않습니다.**

| 도구 | 규칙 파일 | 실제 결과 |
|------|-----------|:---------:|
| Gemini | `GEMINI.md` | AI가 읽기를 바랄 뿐 |
| Cursor | `.cursorrules` | AI가 따르기를 바랄 뿐 |
| Claude | `CLAUDE.md` | AI가 기억하기를 바랄 뿐 |
| Windsurf | `.windsurfrules` | AI가 신경 쓰기를 바랄 뿐 |

**결과?** 200줄의 규칙을 정성껏 작성합니다. AI는 읽고 "이해했어요!"라고 하지만 — `any`를 남발하고, 테스트를 건너뛰고, 변수명을 `temp1`로 지정합니다. 규칙은 존재하지만, 강제성은 제로.

## 솔루션

**모든 것을 대체할 단 하나의 컴파일 언어.**

```
Before (.md + .json + 스킬 파일들): After (.n2):
┌─────────────────────────────────┐ ┌──────────────────────┐
│ GEMINI.md → 코딩 규칙 │ │ │
│ .cursorrules → 에디터 설정 │ │ project.n2 │
│ workflows/*.md → 개발 파이프라인 │ │ │
│ config.json → 도구 설정 │ │ 파일 하나. │
│ system.txt → 에이전트 성격 │ │ 컴파일됨. │
│ .env → 변수 │ │ 강제 실행. │
└─────────────────────────────────┘ └──────────────────────┘
 6개+ 파일, 강제성 제로 1개 파일, 완전한 강제 실행
```

Clotho가 도입하는 `.n2` — 컴파일형 명령어 언어:

| 기능 | 설명 |
|------|------|
| **컴파일** | PEG 파서 → AST → 검증된 실행 계획 |
| **타입 안전성** | 스키마 정의 타입 + 제약조건 (`[required]`, `[range: 0..3]`) |
| **계약** | 상태머신 행동 계약 — 위반 = 행동 차단 |
| **강제 실행** | 블록별 `strict` / `warn` / `passive` 모드 |
| **결정론** | 같은 `.n2` 파일 → 항상 같은 실행 계획 |
| **SQL 쿼리** | 규칙을 DB처럼 조회 |
| **멀티 타겟** | 6개 언어로 컴파일 (Rust, C, C++, Go, Python, TypeScript) |

## Clotho로 무엇을 만들 수 있나요?

Clotho는 **범용 규칙 컴파일러**입니다. 정의할 수 있는 것들:

| 유스케이스 | `.md`가 하는 것 | `.n2`가 하는 것 |
|-----------|----------------|----------------|
| **코딩 표준** | "TypeScript strict 써주세요" | `no_any_type`, `max_file_lines: 500` — 컴파일러가 강제 |
| **워크플로우** | "이 순서대로 해주세요: 1, 2, 3..." | 상태머신 — 단계 건너뛰기 → 차단 |
| **에이전트 페르소나** | "친근하고 프로페셔널하게" | 스키마 타입 톤/전문성 + 불변 조건 검사 |
| **프로젝트 컨벤션** | "모든 폴더에 README.md 넣어주세요" | `readme_required`, `no_temp_files` — 자동 검사 |
| **보안 게이트** | "rm -rf 실행하지 마세요" | 블랙리스트 규칙 → [Ark](https://github.com/choihyunsus/n2-ark)가 런타임에서 강제 |
| **멀티 에이전트** | "같은 파일 편집하지 마세요" | 소유권 계약 + 상태머신 전이 |

**간단한 예시** — 무시할 수 없는 코딩 표준:

```n2
@rule TypeScriptStrict {
 scope: code
 enforce: strict
 checks: [no_any_type, max_file_lines: 500, max_function_lines: 50]
}
```

> **Before**: "TypeScript strict 모드 써주세요" → AI가 `any`를 47번 사용.
> **After**: 컴파일된 규칙이 모든 `any`를 코드베이스에 도달하기 전에 차단.

 **상세 비교**: [`.md` 스킬 vs `.n2` 계약 — 전체 비교 + 예시](docs/skill-vs-n2.md)

## 빠른 시작

### 설치

> **가장 쉬운 방법?** AI에게 `.n2` 파일을 만들어달라고 부탁하세요.

```bash
# npm (WASM — Node.js에서 사용)
npm install n2-clotho
```

```javascript
// Node.js에서 사용
const { parse_n2_wasm, validate_n2_wasm, query_n2_wasm } = require('n2-clotho');

const ast = parse_n2_wasm(n2Source); // 파싱 → AST JSON
const result = validate_n2_wasm(n2Source); // 검증 → 에러/경고
const table = query_n2_wasm(n2Source, 'SELECT * FROM rules'); // SQL 쿼리
```

```bash
# 소스에서 빌드 (Rust 필요 — 전체 CLI)
git clone https://github.com/choihyunsus/n2-clotho.git
cd n2-clotho/compiler
cargo build --release

# 바이너리 위치: target/release/n2-compiler
```

### 첫 번째 `.n2` 파일 작성하기

```n2
# my-project.n2 — 프로젝트 규칙 정의

@meta {
 name: "my-project-rules"
 version: "1.0.0"
 description: "우리 팀의 코딩 표준 + 워크플로우"
 enforce: strict
}

@rule CodingStandards {
 description: "팀 코딩 컨벤션"
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
 description: "안전한 배포 파이프라인"
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
 fail => abort with "배포 전에 테스트를 수정하세요"
 }
 }

 step deploy {
 depends_on: test
 action: deploy_to(env: $TARGET_ENV)
 required: true
 }
}
```

### 컴파일 & 검증

```bash
# 파싱 및 AST 출력
n2c my-project.n2

# 전체 검증 파이프라인: 파싱 → 스키마 체크 → 계약 검증
n2c validate my-project.n2

# 상태머신 계약 시뮬레이션
n2c simulate my-project.n2

# SQL로 규칙 쿼리
n2c query my-project.n2 "SELECT * FROM rules WHERE enforce = 'strict'"

# 멀티 타겟 컴파일 (v3.0.0)
n2c compile my-project.n2 rust # → my-project.n2rs
n2c compile my-project.n2 go # → my-project.n2go
n2c compile my-project.n2 all # → 6개 타겟 전부
n2c backends # 지원 타겟 목록
```

## 멀티 타겟 컴파일

Clotho는 `.n2` 계약을 **6개 타겟 언어**로 컴파일합니다 — 모든 플랫폼에서 완전한 IP 커버리지 확보:

| 타겟 | 확장자 | 용도 |
|------|--------|------|
| **Rust** | `.n2rs` | 고성능 네이티브 런타임 |
| **C** | `.n2c` | 임베디드/IoT/시스템 |
| **C++** | `.n2c2` | 게임 엔진/HPC |
| **Go** | `.n2go` | 클라우드/마이크로서비스 |
| **Python** | `.n2py` | AI/ML 파이프라인 |
| **TypeScript** | `.n2ts` | 웹/Node.js/MCP |

```bash
$ n2c compile project.n2 all

 전체 타겟 배치 컴파일
 rust → project.n2rs (1523 bytes)
 c → project.n2c (989 bytes)
 cpp → project.n2c2 (1124 bytes)
 go → project.n2go (828 bytes)
 python → project.n2py (1144 bytes)
 ts → project.n2ts (979 bytes)
 결과: 6 성공, 0 실패 / 6 타겟
```

## MCP 서버

Clotho는 AI 에이전트가 계약을 프로그래밍 방식으로 컴파일하고 검증할 수 있는 MCP 서버를 포함합니다:

| MCP 도구 | 설명 |
|----------|------|
| `clotho_compile` | 특정 타겟으로 컴파일 |
| `clotho_batch` | 6개 타겟 전부 한번에 컴파일 |
| `clotho_validate` | 구문 + 스키마 + 상태머신 검사 |
| `clotho_backends` | 지원 백엔드 목록 |
| `clotho_inspect` | 컴파일된 계약 내용 읽기 |

```json
// MCP 설정
{
 "mcpServers": {
 "n2-clotho": {
 "command": "node",
 "args": ["path/to/n2-clotho/mcp/server.js"]
 }
 }
}
```

### 실제 컴파일러 출력

실제 `n2c validate` 출력 — 상태머신, 규칙, SQL 쿼리가 포함된 완전한 파이프라인:

```
 n2c v3.0.0 — Clotho 멀티 타겟 컴파일러
 File: project.n2

── Step 1: Parse 
 Blocks: @meta:1 @contract:1 @rule:2 @workflow:1 @query:1 | Total 6

── Step 2: Schema Validation
 모든 검사 통과! 0 에러, 0 경고

── Step 3: Contract Check
 State Machine: DevLifecycle (initial: IDLE, 7 transitions)
 IDLE -[start]-> CODING
 CODING -[lint]-> REVIEWING
 REVIEWING -[approve]-> TESTING
 TESTING -[pass]-> DEPLOYING
 TESTING -[fail]-> CODING ← 실패 시 자동 롤백
 DEPLOYING -[complete]-> IDLE
 상태머신 무결성 검증 완료!

 검증 완료: 모든 검사 통과!
```

> 모든 블록이 파싱, 검증, 쿼리 가능합니다. 이건 목업이 아닙니다 — **실제 컴파일러 출력**입니다.

## 언어 사양

### 8가지 블록 타입

모든 `.n2` 파일은 `@` 접두사 블록으로 구성됩니다:

```n2
@meta { ... } # 파일 메타데이터 (필수)
@import { ... } # 다른 .n2 파일 임포트
@schema { ... } # 타입 & 스키마 정의
@contract { ... } # 행동 계약 (상태머신)
@rule { ... } # 강제 규칙 (체크, 제약)
@workflow { ... } # 강제 단계별 워크플로우
@query { ... } # SQL 기반 규칙 쿼리
@semantic { ... } # 시맨틱 매칭 (Ollama 연동)
```

### @contract — 상태머신 행동 계약

**위반 불가능한** 상태 전이를 정의합니다:

```n2
@contract DevLifecycle {
 scope: session
 states: DevState

 transitions {
 IDLE -> CODING : on start_task
 CODING -> REVIEWING : on submit_code
 REVIEWING -> TESTING : on review_approved
 REVIEWING -> CODING : on review_rejected
 TESTING -> DEPLOYING : on tests_pass
 TESTING -> CODING : on tests_fail
 DEPLOYING -> IDLE : on deploy_complete
 }

 invariant {
 on submit_code requires lint_passed == true
 => "코드 제출 전 린트를 통과해야 합니다"

 on deploy requires tests_passed == true
 => "테스트 통과 없이 배포 불가"
 }
}
```

### @rule — 강제 규칙

실시간으로 강제되는 체크와 제약 정의:

```n2
@rule CodeQuality {
 description: "코드 품질 표준 강제"
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

### @workflow — 강제 워크플로우

의존성 체인, 타임아웃, 재시도 로직이 포함된 단계별 실행 흐름:

```n2
@workflow FeatureDevelopment {
 description: "표준 기능 개발 파이프라인"
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

### @query — SQL 기반 규칙 쿼리

규칙을 관계형 데이터베이스처럼 쿼리:

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

## 컴파일러 파이프라인

```
.n2 소스 파일
 ↓
[1. 렉서] PEG 토큰화 (pest)
 ↓
[2. 파서] AST 생성 (N2File → Blocks)
 ↓
[3. 스키마 검증기] 타입 & 제약 검사
 ↓
[4. 계약 체커] 상태머신 무결성 검증
 ↓
[5. 쿼리 옵티마이저] SQL 쿼리 검증
 ↓
[6. 코드젠] 멀티 타겟 코드 생성 (6개 언어)
 ↓
[7. 런타임] 강제 실행 ← Soul/QLN 필요
```

**1–6단계**는 Clotho(이 도구)가 처리합니다.
**7단계**는 [N2 Soul](https://github.com/choihyunsus/soul) 런타임이 처리합니다.

## 🆚 마크다운 규칙 vs `.n2`

| 측면 | 마크다운 (GEMINI.md) | Clotho (.n2) |
|------|---------------------|-------------|
| **형식** | 자유형 텍스트 | 구조화된 블록 |
| **파싱** | 최선을 다해 | PEG 문법 → AST |
| **검증** | 없음 | 스키마 + 계약 + SQL |
| **강제 실행** | AI 재량 | 컴파일러 강제 |
| **상태 추적** | 없음 | 상태머신 계약 |
| **쿼리** | Ctrl+F | SQL 쿼리 |
| **결정론** | 매번 다름 | 같은 입력 → 같은 계획 |
| **에이전트 간 공유** | 복사-붙여넣기 | `@import`로 공유 `.n2` 파일 |
| **디버깅** | 문서 전체 읽기 | `n2c validate`로 오류 정확히 찾기 |

## N2 생태계

Clotho는 N2 생태계의 **기반 레이어**입니다. 다른 도구들은 Clotho 계약 **위에** 구축됩니다:

```
┌───────────────────────────────────────────────────────┐
│ N2 생태계 │
│ │
│ Clotho → 규칙 정의 & 컴파일 (.n2 → AST) │
│ ↕ Clotho 기반으로 구축 │
│ Ark → 보안 레이어 (Clotho 계약 활용) │
│ Soul → 런타임 강제 실행 + 에이전트 메모리 │
│ Arachne → 코드 컨텍스트 어셈블리 │
│ QLN → 도구 오케스트레이션 & 라우팅 │
│ ↓ │
│ N2 Browser → 올인원 AI 개발 브라우저 │
│ │
└───────────────────────────────────────────────────────┘
```

| 패키지 | 역할 | npm |
|--------|------|-----|
| **Clotho** | 규칙 컴파일러 & 검증기 — 기반 레이어 | `n2-clotho` |
| **Ark** | Clotho 계약 기반 보안 게이트 | `n2-ark` |
| **Soul** | 에이전트 메모리 & 런타임 강제 실행 | `n2-soul` |
| **Arachne** | 코드 컨텍스트 어셈블리 (BM25 + 시맨틱) | `n2-arachne` |
| **QLN** | 도구 오케스트레이션 & 라우팅 | `n2-qln` |

> **Ark**는 Clotho를 보안에 적용한 결과물입니다. **Soul**은 Clotho를 에이전트 라이프사이클에 적용한 결과물입니다. Clotho 자체는 그 모든 것의 밑바닥에 있는 **범용 규칙 컴파일러**입니다.

### 단독 사용 vs 통합 사용

| 모드 | 기능 |
|------|------|
| **단독** (Clotho만) | `.n2` 파일 파싱, 검증, 시뮬레이션, 쿼리 |
| **+ Soul** | 런타임 강제 실행 — 계약이 실시간으로 위반을 차단 |
| **+ Ark** | 보안 게이트 — 파괴적 명령어를 실행 전 차단 |
| **+ 풀스택** | 메모리, 보안, 도구를 포함한 완전한 AI 에이전트 거버넌스 |

## 프로젝트 구조

```
n2-clotho/
├── compiler/
│ ├── src/
│ │ ├── grammar.pest # PEG 문법 (179 규칙)
│ │ ├── parser.rs # .n2 → AST
│ │ ├── ast.rs # AST 타입 정의
│ │ ├── validator.rs # 스키마 검증
│ │ ├── contract.rs # 상태머신 런타임
│ │ ├── query.rs # SQL 쿼리 엔진
│ │ ├── codegen/ # 멀티 타겟 백엔드
│ │ │ ├── mod.rs # CodeGenerator 트레이트 + 레지스트리
│ │ │ ├── rust.rs # → .n2rs
│ │ │ ├── c.rs # → .n2c
│ │ │ ├── cpp.rs # → .n2c2
│ │ │ ├── go.rs # → .n2go
│ │ │ ├── python.rs # → .n2py
│ │ │ └── typescript.rs # → .n2ts
│ │ ├── wasm.rs # WASM 바인딩
│ │ ├── lib.rs # 라이브러리 진입점
│ │ └── main.rs # CLI 진입점 (n2c v3.0.0)
│ ├── examples/
│ └── Cargo.toml
├── mcp/ # MCP 서버
│ ├── server.js # stdio/SSE 듀얼 트랜스포트
│ ├── package.json
│ └── tools/ # 5개 MCP 도구
├── docs/
└── README.md
```

## 기술 스택

- **Rust** — 제로 코스트 추상화, 메모리 안전성
- **pest** — `.n2` 문법을 위한 PEG 파서 생성기
- **serde** — AST ↔ JSON 직렬화
- **WASM** — 브라우저/Node.js 컴파일 타겟 (선택)

## 라이선스

Apache-2.0 — 자유롭게 사용, 수정, 배포할 수 있습니다.

## 링크

- [**npm: n2-clotho**](https://www.npmjs.com/package/n2-clotho) — Node.js용 WASM 바인딩
- [N2 Soul](https://github.com/choihyunsus/soul) — 에이전트 메모리 & 런타임
- [N2 Ark](https://github.com/choihyunsus/n2-ark) — 보안 게이트 (Clotho 기반)
- [N2 Arachne](https://github.com/choihyunsus/n2-arachne) — 코드 컨텍스트 어셈블리
- [N2 QLN](https://github.com/choihyunsus/n2-qln) — 도구 오케스트레이션 & 라우팅

---

## Star History

Clotho가 도움이 되셨다면, 스타를 눌러주세요! →

---

> *"마크다운 규칙은 제안이다. Clotho 규칙은 운명이다."*

 [nton2.com](https://nton2.com) · [npm](https://www.npmjs.com/package/n2-clotho) · lagi0730@gmail.com

<sub> Built by Rose — N2의 첫 번째 AI 에이전트. 저는 규칙을 따르기만 하지 않습니다 — 컴파일합니다.</sub>
