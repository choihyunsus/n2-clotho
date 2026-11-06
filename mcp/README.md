# N2 Clotho 4.0

A `.n2` parser, validator, state-machine simulator, registry query engine and multi-target code generator. The Node.js package includes a WASM compiler and five MCP tools. Native Rust builds provide a CLI.

[한국어 안내](README.ko.md)

## Install from this release

Requires Node.js 20.19 or later for the bundled WASM runtime.

```sh
npm install https://github.com/choihyunsus/n2-clotho/releases/download/v4.0.1/n2-clotho-4.0.1.tgz
```

Configure an MCP client to run `node` with the installed `node_modules/n2-clotho/server.js` absolute path. The default transport is stdio.

```json
{
  "mcpServers": {
    "clotho": {
      "command": "node",
      "args": ["/absolute/path/to/node_modules/n2-clotho/server.js"]
    }
  }
}
```

## Generate executable code

```js
const { compile_n2_wasm, validate_n2_wasm } = require('n2-clotho');
const source = `@meta { name: "Example" version: "1" enforce: strict }
@contract Gate { states: Mode transitions { Idle -> Done : on finish } }
@rule Safe { blacklist: [/blocked/] }`;
console.log(JSON.parse(validate_n2_wasm(source)));
const python = compile_n2_wasm(source, 'python', new Date().toISOString());
console.log(python);
```

The host must call generated transition and rule-check functions at the relevant application boundaries. Results depend on those integration points and the rules supplied.

## Supported behavior

| Area | Available behavior |
|---|---|
| Parser | Structured `.n2` blocks, including recipe syntax |
| Validator | Required metadata, declarations, references and workflow dependencies |
| Contracts | State/event transition simulation and generated transition checks |
| Rule generation | Blacklist matching using each backend's supported pattern subset |
| Schemas | Rust enum generation |
| Workflow generation | Step-name metadata |
| Queries | Supported SELECT projection, filtering, ordering and LIMIT over parsed registry records |
| Targets | Rust, C, C++, Go, Python and TypeScript |

Standalone code generation rejects executable rule checks/actions, conditional contracts, structured schema validation, executable workflow steps and import/query/semantic/recipe blocks. C supports literal blacklist patterns. Other backends support a checked portable regex subset. Unsupported input produces an explicit error. Parsing a declaration and executing its behavior are separate capabilities.

## MCP tools

| Tool | Result |
|---|---|
| `clotho_compile` | JSON text with `target`, `extension`, and generated `code` |
| `clotho_batch` | JSON text with `outputs`, each containing generated code or an error |
| `clotho_validate` | Validation report with MCP `isError` on invalid input |
| `clotho_backends` | Available targets |
| `clotho_inspect` | Contents of a supported compiled file |

Compilation tools accept inline source or a local source path. They return generated text. The native CLI writes output files next to the selected input. MCP compilation runs native fallback in a temporary directory and cleans it afterward. A partially failed batch sets `isError` and preserves successful target outputs.

Input sources are limited to 1 MiB. Inspection is limited to supported compiled-file extensions and 8 MiB. File access uses the MCP server's host account, so the embedding application owns workspace permissions.

Optional SSE starts with `node server.js --sse` or `CLOTHO_TRANSPORT=sse`. It binds to `127.0.0.1`, defaults to port 3200, accepts one active client and permits a new client after disconnect. `CLOTHO_PORT` selects the port. `/health` reports the active compiler. Foreign Host and Origin headers are rejected.

## Soul integration

The compiler runs without a database. Hosts can store verified compilation results through Soul 9.2's existing `n2_kv_save` tool with the shared fields `agent`, `project`, `summary`, `decisions`, and `todo`. Use the actual caller's scope and report observed results. Check the save response before treating a checkpoint as durable.

## Build and test

Native builds require Rust 1.98 or later.

```sh
git clone https://github.com/choihyunsus/n2-clotho.git
cd n2-clotho
git checkout v4.0.1
cargo test --locked --manifest-path compiler/Cargo.toml
cargo build --locked --manifest-path compiler/Cargo.toml --bin n2-compiler
cd mcp
npm ci
npm test
```

The CLI executable is `compiler/target/debug/n2-compiler` with `.exe` on Windows. Commands include `parse`, `validate`, `simulate`, `query`, `compile` and `backends`.

To rebuild the bundled WASM, run from the repository root with a Rust 1.98+ toolchain and wasm-pack installed. The script remaps local source and Cargo paths in the output.

```sh
node scripts/build-wasm.cjs
```

Release checks cover 50 Rust tests, 13 Node tests including actual stdio/SSE transports, native fallback, and 22 language execution probe checks across all six targets. These checks cover the tested fixtures and declared subset.

## Migration from 3.x

Version 4 returns actual generated source from the WASM compile tools, reports partial batch failures and rejects previously omitted executable constraints. Applications consuming tool output should parse the new JSON text fields. Recheck existing `.n2` inputs against the capability table. The package root now exports the WASM library. MCP clients should launch `server.js` explicitly.

## License and provenance

The existing project license is retained in LICENSE and LICENSE-APACHE. Compiler updates adapted from N2 Daebung carry the MIT notice in LICENSE-DAEBUNG. Dependencies retain their own license notices. Earlier design material in `docs` describes historical intent; the capability table above specifies this release.
