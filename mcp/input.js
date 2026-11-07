// Validate Unicode and bound file reads before strings cross the compiler boundary.
const fs = require('node:fs');
const path = require('node:path');
const MAX_SOURCE_BYTES = 1024 * 1024;

function checkedText(value, maximum, label, allowNul = false) {
    if (typeof value !== 'string') throw new Error(`${label} must be text`);
    if (value.length > maximum || Buffer.byteLength(value) > maximum) {
        const limit = maximum % MAX_SOURCE_BYTES === 0 ? `${maximum / MAX_SOURCE_BYTES} MiB` : `${maximum} bytes`;
        throw new Error(`${label} exceeds ${limit}`);
    }
    if ((!allowNul && value.includes('\0')) || Buffer.from(value).toString('utf8') !== value)
        throw new Error(`${label} must contain well-formed UTF-8${allowNul ? '' : ' without NUL'}`);
    return value;
}

function readUtf8File(filename, maximum) {
    checkedText(filename, 8192, 'File path');
    if (/^\\\\[.?]\\/u.test(filename)) throw new Error('A regular file path is required');
    const resolved = fs.realpathSync.native(filename);
    if (!fs.statSync(resolved).isFile()) throw new Error('A regular file is required');
    const flags = fs.constants.O_RDONLY | (fs.constants.O_NONBLOCK ?? 0) | (fs.constants.O_NOFOLLOW ?? 0);
    const handle = fs.openSync(resolved, flags);
    const buffer = Buffer.allocUnsafe(maximum + 1);
    let length = 0;
    try {
        const info = fs.fstatSync(handle);
        if (!info.isFile() || info.size > maximum) throw new Error(`File exceeds ${maximum} bytes or is not regular`);
        while (length < buffer.length) {
            const read = fs.readSync(handle, buffer, length, buffer.length - length, null);
            if (!read) break;
            length += read;
        }
    } finally { fs.closeSync(handle); }
    if (length > maximum) throw new Error(`File grew beyond ${maximum} bytes`);
    const text = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(buffer.subarray(0, length));
    return checkedText(text, maximum, 'File');
}

function readSource(source) {
    checkedText(source, MAX_SOURCE_BYTES, 'Source');
    if (!source.trim()) throw new Error('Source is required');
    if (source.includes('\n') || source.trimStart().startsWith('@')) return source;
    return readUtf8File(source, MAX_SOURCE_BYTES);
}

function removeOwnedTemporary(directory, temporaryRoot) {
    const resolved = fs.realpathSync.native(directory), root = fs.realpathSync.native(temporaryRoot);
    if (path.dirname(resolved) !== root || !path.basename(resolved).startsWith('n2-clotho-'))
        throw new Error('Temporary compiler directory ownership changed');
    fs.rmSync(resolved, { recursive: true, force: true });
}
module.exports = { MAX_SOURCE_BYTES, checkedText, readUtf8File, readSource, removeOwnedTemporary };
