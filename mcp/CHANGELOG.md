# 4.1.0

- Add bounded Rust Unicode regular-expression predicates through native and WASM APIs.
- Expose the predicate and compiler functions with CJS, ESM and TypeScript consumer coverage.
- Reject malformed Unicode and NUL in compiler source before compilation.
- Read regular UTF-8 files through bounded handles, including files that grow during a read.
- Verify one release archive on Windows, Linux and macOS after source checks.

# 4.0.1

- Update the MCP runtime to SDK 1.32.2 and Zod 4.6.7, including patched transitive dependencies.
- Verify exported tool schemas and reject malformed arguments over both stdio and SSE.
- Rebuild the WASM compiler with matching 4.0.1 version metadata.
- Add Windows, Linux and macOS release checks with production dependency auditing.

# 4.0.0

- Port validated parser, query, contract and generator fixes from Daebung.
- Generate real code through WASM compile tools and expose the library at the package root.
- Preserve per-target failures and reject unsupported executable semantics.
- Correct native batch exit statuses and validate state-machine integrity.
- Exercise real MCP transports, loopback SSE lifecycle and native fallback.
- Document supported behavior and Soul checkpoint field compatibility.
