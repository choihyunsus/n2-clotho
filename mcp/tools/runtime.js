// Shared source handling and bounded native execution for compiler tools.
const { execFileSync } = require('node:child_process');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');

const EXTENSIONS = { rust: '.n2rs', c: '.n2c', cpp: '.n2c2', go: '.n2go', python: '.n2py', ts: '.n2ts' };
const MAX_SOURCE_BYTES = 1024 * 1024;

function readSource(source) {
    if (typeof source !== 'string' || !source.trim()) throw new Error('Source is required');
    if (Buffer.byteLength(source) > MAX_SOURCE_BYTES) throw new Error('Source exceeds 1 MiB');
    if (source.includes('\n') || source.trimStart().startsWith('@')) return source;
    const stat = fs.statSync(source);
    if (!stat.isFile() || stat.size > MAX_SOURCE_BYTES) throw new Error('Source must be a file of at most 1 MiB');
    const text = fs.readFileSync(source, 'utf8');
    if (Buffer.byteLength(text) > MAX_SOURCE_BYTES) throw new Error('Source exceeds 1 MiB');
    return text;
}

function runNative(compiler, command, source, target) {
    const temporary = fs.mkdtempSync(path.join(os.tmpdir(), 'n2-clotho-'));
    try {
        const file = path.join(temporary, 'input.n2');
        fs.writeFileSync(file, source, 'utf8');
        const args = [command, file];
        if (target) args.push(target);
        const output = execFileSync(compiler.bin, args, { encoding: 'utf8', timeout: 30000, maxBuffer: 8 * 1024 * 1024 });
        if (command === 'compile') return fs.readFileSync(path.join(temporary, `input${EXTENSIONS[target]}`), 'utf8');
        return output;
    } finally {
        fs.rmSync(temporary, { recursive: true, force: true });
    }
}

function compileSource(compiler, source, target) {
    if (!Object.hasOwn(EXTENSIONS, target)) throw new Error('Unknown target');
    if (compiler.type === 'wasm') return compiler.module.compile_n2_wasm(source, target, new Date().toISOString());
    return runNative(compiler, 'compile', source, target);
}

function errorResult(error) {
    const message = error?.stderr ? error.stderr.toString() : String(error?.message ?? error);
    return { content: [{ type: 'text', text: message }], isError: true };
}

module.exports = { EXTENSIONS, readSource, runNative, compileSource, errorResult };
