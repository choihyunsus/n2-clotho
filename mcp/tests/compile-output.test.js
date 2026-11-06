// Verify real WASM output, executable TypeScript and MCP error propagation.
const { test } = require('node:test');
const assert = require('node:assert/strict');
const vm = require('node:vm');
const ts = require('typescript');
const { z } = require('zod');
const wasm = require('../wasm/n2_compiler.js');
const pkg = require('../package.json');
const { compileSource, readSource } = require('../tools/runtime');
const { registerCompileTools } = require('../tools/compile');
const { registerBatchTools } = require('../tools/batch');
const { registerValidateTools } = require('../tools/validate');
const compiler = { type: 'wasm', module: wasm };
const source = `@meta { name: "Probe" version: "1" enforce: strict }
@contract Gate { states: Mode transitions { Idle -> Ready : on boot Ready -> Done : on end } }
@rule Safe { blacklist: [/blocked/] }`;
const handlers = {};
const server = { tool(name, description, schema, handler) { handlers[name] = handler; } };
registerCompileTools(server, z, compiler);
registerBatchTools(server, z, compiler);
registerValidateTools(server, z, compiler);

for (const target of ['rust', 'c', 'cpp', 'go', 'python', 'ts']) {
    test(`WASM generates ${target} with matching version and metadata`, () => {
        const code = compileSource(compiler, source, target);
        assert.ok(code.includes('Gate') || code.includes('gate'));
        assert.ok(code.includes('blocked'));
        assert.ok(code.includes(`n2c v${pkg.version}`));
        assert.match(code, /Compiled:.*\d{4}-\d{2}-\d{2}T/);
        if (target === 'c') assert.match(code, /#endif/);
    });
}

test('generated TypeScript actually enforces transitions and blacklist matches', () => {
    const code = compileSource(compiler, source, 'ts');
    const javascript = ts.transpileModule(code, { compilerOptions: {
        target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText;
    const sandbox = { exports: {} };
    vm.runInNewContext(javascript, sandbox, { timeout: 1000 });
    const { Gate, SafeRule } = sandbox.exports;
    const gate = new Gate();
    assert.equal(gate.currentState, 'Idle');
    assert.throws(() => gate.transition('end'));
    assert.equal(gate.currentState, 'Idle');
    assert.equal(gate.transition('boot'), 'Ready');
    assert.equal(gate.transition('end'), 'Done');
    assert.throws(() => gate.transition('boot'));
    assert.equal(gate.currentState, 'Done');
    assert.ok(SafeRule.checkBlacklist('blocked'));
    assert.equal(SafeRule.checkBlacklist('allowed'), null);
});

test('compile and batch tools return generated code', async () => {
    const result = await handlers.clotho_compile({ source, target: 'python' });
    assert.ok(!result.isError);
    assert.match(JSON.parse(result.content[0].text).code, /class Gate/);
    const batch = await handlers.clotho_batch({ source });
    assert.ok(!batch.isError);
    assert.equal(JSON.parse(batch.content[0].text).outputs.filter(item => item.code).length, 6);
});

test('unsupported executable constraints fail every target', async () => {
    const unsupported = '@meta { name: "Probe" enforce: strict } @rule Safe { check { $input.ready == true => "required" } }';
    const result = await handlers.clotho_batch({ source: unsupported });
    assert.equal(result.isError, true);
    assert.equal(JSON.parse(result.content[0].text).outputs.filter(item => item.error).length, 6);
    const invalid = await handlers.clotho_validate({ source: '@meta {' });
    assert.equal(invalid.isError, true);
    const missing = await handlers.clotho_compile({ source: 'missing-file.n2', target: 'ts' });
    assert.equal(missing.isError, true);
});

test('partial batches report actual errors alongside successful outputs', async () => {
    const regexSource = '@meta { name: "Probe" enforce: strict } @rule Safe { blacklist: [/secret[0-9]+/i] }';
    const batch = await handlers.clotho_batch({ source: regexSource });
    assert.equal(batch.isError, true);
    const outputs = JSON.parse(batch.content[0].text).outputs;
    assert.ok(outputs.find(item => item.target === 'c').error);
    assert.ok(outputs.find(item => item.target === 'python').code);
});

test('validation errors propagate and source input is bounded', async () => {
    const result = await handlers.clotho_validate({ source: '@rule Safe { blacklist: [/blocked/] }' });
    assert.equal(result.isError, true);
    assert.throws(() => readSource('x'.repeat(1024 * 1024 + 1)), /1 MiB/);
    assert.throws(() => readSource(''), /required/);
    assert.match(wasm.n2c_version(), new RegExp(pkg.version.replaceAll('.', '\\.')));
});
