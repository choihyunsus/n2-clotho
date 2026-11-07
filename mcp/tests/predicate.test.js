// Exercise the public predicate and text boundaries using actual WASM and bounded local files.
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const compiler = require('..');
const { readSource, readUtf8File, removeOwnedTemporary } = require('../input');
const { registerInspectTools } = require('../tools/inspect');
const { compileSource, runNative } = require('../tools/runtime');
const { z } = require('zod');

function temporary(t) {
    const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'n2-clotho-predicate-'));
    t.after(() => removeOwnedTemporary(directory, os.tmpdir()));
    return directory;
}

test('public predicates retain Rust Unicode semantics, flags and explicit release', () => {
    const word = new compiler.BoundedRegex('^\\w+$', 'u');
    const line = new compiler.BoundedRegex('^secret$', 'igm');
    const nul = new compiler.BoundedRegex('\\x00');
    try {
        assert.equal(word.test('한글_123'), true);
        assert.equal(word.test('two words'), false);
        assert.equal(line.test('before\nSECRET\nafter'), true);
        assert.equal(line.test('SECRET'), true);
        assert.equal(line.test('SECRET'), true);
        assert.equal(nul.test('a\0b'), true);
    } finally { word.free(); line.free(); nul.free(); }
    word.free();
    assert.throws(() => word.test('late'), /freed/);
});

test('invalid predicates, large input and malformed UTF-16 fail explicitly', () => {
    for (const pattern of ['(a)\\1', '(?=a)', '[', 'a'.repeat(4097), '\ud800'])
        assert.throws(() => new compiler.BoundedRegex(pattern));
    assert.throws(() => new compiler.BoundedRegex('x', 'z'));
    const regex = new compiler.BoundedRegex('(a+)+$');
    try {
        assert.equal(regex.test('a'.repeat(10000) + '!'), false);
        assert.throws(() => regex.test('x'.repeat(1024 * 1024 + 1)));
        assert.throws(() => regex.test('\ud800'));
    } finally { regex.free(); }
    assert.throws(() => compiler.parse_n2_wasm('@meta { name: "\ud800" }'), /UTF-8/);
    assert.throws(() => compiler.validate_n2_wasm('x'.repeat(1024 * 1024 + 1)), /1 MiB/);
    assert.equal(JSON.parse(compiler.parse_n2_wasm('@meta { name: "한글😀" }')).blocks[0].fields[0].value, '한글😀');
});

test('raw compiler helpers reject damaged source before entering WASM or spawning native code', () => {
    const raw = { type: 'wasm', module: require('../wasm/n2_compiler') };
    for (const source of ['\ud800', 'x\0y', 'x'.repeat(1024 * 1024 + 1)]) {
        assert.throws(() => compileSource(raw, source, 'ts'), /UTF-8|1 MiB/);
        assert.throws(() => runNative({ bin: 'must-not-launch' }, 'validate', source), /UTF-8|1 MiB/);
    }
});

test('source and inspection reads reject damaged UTF-8 and retain exact valid file text', async (t) => {
    const directory = temporary(t), valid = path.join(directory, 'valid.n2'), invalid = path.join(directory, 'invalid.n2ts');
    fs.writeFileSync(valid, '\ufeff@meta { name: "한글😀" }');
    assert.equal(readSource(valid), fs.readFileSync(valid, 'utf8'));
    fs.writeFileSync(invalid, Buffer.from([0xff, 0x61]));
    assert.throws(() => readSource(invalid), /encoded data|UTF-8/);
    assert.throws(() => readUtf8File(directory, 1024), /regular/);
    const handlers = {};
    registerInspectTools({ tool(name, _description, _schema, handler) { handlers[name] = handler; } }, z, {});
    assert.equal((await handlers.clotho_inspect({ file: invalid })).isError, true);
});

test('a source file growing after inspection is still bounded by the open-handle reader', (t) => {
    const directory = temporary(t), file = path.join(directory, 'growing.n2');
    fs.writeFileSync(file, 'before');
    const original = fs.readSync;
    let changed = false;
    t.mock.method(fs, 'readSync', (...args) => {
        if (!changed) { changed = true; fs.appendFileSync(file, 'x'.repeat(100)); }
        return original(...args);
    });
    assert.throws(() => readUtf8File(file, 32), /grew beyond/);
});

test('a FIFO is refused before a blocking open', { skip: process.platform === 'win32' }, (t) => {
    const directory = temporary(t), fifo = path.join(directory, 'input.n2');
    const made = spawnSync('mkfifo', [fifo], { encoding: 'utf8' });
    assert.equal(made.status, 0, made.stderr);
    assert.throws(() => readSource(fifo), /regular/);
});
