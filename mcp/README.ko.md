# N2 Clotho 4.0

`.n2` 파싱·검증·상태 전이 시뮬레이션·레지스트리 조회·다중 언어 코드 생성을 제공합니다. Node.js 패키지에는 WASM 컴파일러와 MCP 도구 5개가 들어 있습니다.

## 설치

Node.js 20.19 이상이 필요합니다.

```sh
npm install https://github.com/choihyunsus/n2-clotho/releases/download/v4.0.1/n2-clotho-4.0.1.tgz
```

MCP 클라이언트에서 `node`와 설치된 `node_modules/n2-clotho/server.js`의 절대 경로를 지정합니다. 기본 통신은 stdio입니다.

## 지원 범위

상태·이벤트 전이, 블랙리스트 검사와 제한된 코드 생성을 지원합니다. 대상 언어는 Rust·C·C++·Go·Python·TypeScript입니다. Rust는 enum 생성을 지원하고, 워크플로 코드는 단계 이름 메타데이터를 제공합니다.

실행 가능한 rule check/action, 조건식 계약, 구조체 필드 검증, 실행 워크플로, import/query/semantic/recipe 블록의 독립 코드 생성은 명시적으로 거부합니다. C 블랙리스트는 리터럴 패턴을 지원합니다. 다른 대상은 검사된 정규식 부분집합을 지원합니다. 상세 범위는 영문 README 표를 확인하세요.

호스트 애플리케이션이 실제 동작 경계에서 생성된 검사 함수를 호출해야 규칙이 적용됩니다.

## MCP 도구

- `clotho_compile`은 실제 생성 코드가 포함된 `target`, `extension`, `code` JSON을 반환합니다.
- `clotho_batch`는 대상별 코드 또는 오류가 포함된 `outputs` JSON을 반환합니다.
- `clotho_validate`는 검증 결과와 오류 상태를 반환합니다.
- `clotho_backends`는 대상 언어를 표시합니다.
- `clotho_inspect`는 지원하는 확장자의 생성 파일을 읽습니다.

MCP 컴파일은 생성 코드를 반환합니다. 네이티브 CLI는 입력 파일 옆에 결과를 저장합니다. 부분 실패한 배치는 성공한 결과와 오류를 함께 반환합니다. 입력은 1 MiB, 생성 파일 열람은 8 MiB로 제한합니다.

SSE는 `node server.js --sse`로 실행합니다. 127.0.0.1에 바인딩하고 기본 포트는 3200입니다. 동시에 한 클라이언트를 허용하며 종료 후 다시 연결할 수 있습니다. `CLOTHO_PORT`로 포트를 지정합니다.

## Soul 연동

컴파일러는 DB 없이 단독 실행합니다. 검증된 결과의 저장은 호스트에서 Soul 9.2의 `n2_kv_save`를 호출해 처리합니다. 공통 필드 `agent`, `project`, `summary`, `decisions`, `todo`를 사용하고 저장 응답을 확인합니다.

## 검증과 이전 버전에서의 변경

Rust 1.98 이상에서 `cargo test --locked --manifest-path compiler/Cargo.toml`을 실행합니다. `mcp` 폴더에서 `npm ci` 후 `npm test`를 실행합니다.

4.0은 WASM 도구에서 실제 코드를 생성하고, 배치 실패를 정확히 표시하며, 구현되지 않은 실행 조건을 거부합니다. 3.x 사용자는 기존 입력과 응답 파서를 확인해야 합니다. 패키지 루트는 WASM 라이브러리를 내보내고 MCP는 `server.js`를 직접 실행합니다.

기존 프로젝트 라이선스를 유지하며 대붕이에서 이식한 코드의 MIT 고지를 LICENSE-DAEBUNG에 포함합니다. 이전 설계 문서의 계획과 현재 구현 범위는 영문 README의 지원 표로 구분할 수 있습니다.
