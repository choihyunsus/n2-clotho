// n2-compiler 라이브러리 — .n2 언어 파서 + 검증 + 쿼리 + 코드생성 공개 모듈
pub mod ast;
pub mod parser;
pub mod validator;
pub mod contract;
pub mod query;
pub mod codegen;

#[cfg(feature = "wasm")]
pub mod wasm;

mod utils;
#[cfg(test)]
mod semantic_tests;
