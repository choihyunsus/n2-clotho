// Preserve the compiler API while checking text at the JavaScript-to-WASM boundary.
const raw = require('./wasm/n2_compiler');
const { checkedText, MAX_SOURCE_BYTES } = require('./input');
Object.assign(exports, raw);

exports.parse_n2_wasm = source => raw.parse_n2_wasm(checkedText(source, MAX_SOURCE_BYTES, 'Source'));
exports.validate_n2_wasm = source => raw.validate_n2_wasm(checkedText(source, MAX_SOURCE_BYTES, 'Source'));
exports.extract_blacklist_wasm = source => raw.extract_blacklist_wasm(checkedText(source, MAX_SOURCE_BYTES, 'Source'));
exports.query_n2_wasm = (source, sql) => raw.query_n2_wasm(
    checkedText(source, MAX_SOURCE_BYTES, 'Source'), checkedText(sql, 65536, 'Query'));
exports.compile_n2_wasm = (source, target, compiledAt) => raw.compile_n2_wasm(
    checkedText(source, MAX_SOURCE_BYTES, 'Source'), checkedText(target, 64, 'Target'),
    checkedText(compiledAt, 128, 'Compilation timestamp'));
exports.n2c_version = raw.n2c_version;

exports.BoundedRegex = class BoundedRegex {
    #inner;
    constructor(pattern, flags = '') {
        this.#inner = new raw.BoundedRegex(checkedText(pattern, 4096, 'Pattern', true), checkedText(flags, 16, 'Flags'));
    }
    test(input) {
        if (!this.#inner) throw new Error('Predicate was freed');
        return this.#inner.test(checkedText(input, MAX_SOURCE_BYTES, 'Predicate input', true));
    }
    free() {
        this.#inner?.free();
        this.#inner = undefined;
    }
};
