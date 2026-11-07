// Shared source handling and bounded native execution for compiler tools.
const { execFileSync } = require('node:child_process');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { readSource, readUtf8File, checkedText, MAX_SOURCE_BYTES, removeOwnedTemporary } = require('../input');

const EXTENSIONS = { rust: '.n2rs', c: '.n2c', cpp: '.n2c2', go: '.n2go', python: '.n2py', ts: '.n2ts' };

function runNative(compiler, command, source, target) {
    checkedText(source, MAX_SOURCE_BYTES, 'Source');
    const temporary = fs.mkdtempSync(path.join(os.tmpdir(), 'n2-clotho-'));
    try {
        const file = path.join(temporary, 'input.n2');
        fs.writeFileSync(file, source, 'utf8');
        const args = [command, file];
        if (target) args.push(target);
        const output = execFileSync(compiler.bin, args, { encoding: 'utf8', timeout: 30000, maxBuffer: 8 * 1024 * 1024, windowsHide: true });
        if (command === 'compile') return readUtf8File(path.join(temporary, `input${EXTENSIONS[target]}`), 8 * 1024 * 1024);
        return output;
    } finally {
        removeOwnedTemporary(temporary, os.tmpdir());
    }
}

function compileSource(compiler, source, target) {
    checkedText(source, MAX_SOURCE_BYTES, 'Source');
    if (!Object.hasOwn(EXTENSIONS, target)) throw new Error('Unknown target');
    if (compiler.type === 'wasm') return compiler.module.compile_n2_wasm(source, target, new Date().toISOString());
    return runNative(compiler, 'compile', source, target);
}

function errorResult(error) {
    const message = error?.stderr ? error.stderr.toString() : String(error?.message ?? error);
    return { content: [{ type: 'text', text: message }], isError: true };
}

module.exports = { EXTENSIONS, readSource, runNative, compileSource, errorResult };
