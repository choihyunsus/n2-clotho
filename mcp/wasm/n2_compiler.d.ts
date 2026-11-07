/* tslint:disable */
/* eslint-disable */

/**
 * A reusable Rust-regex predicate with explicit pattern, input and automaton limits.
 */
export class BoundedRegex {
    free(): void;
    [Symbol.dispose](): void;
    constructor(pattern: string, flags: string);
    test(input: string): boolean;
}

/**
 * Generate the requested language using the same validated backends as the CLI.
 */
export function compile_n2_wasm(source: string, target: string, compiled_at: string): string;

/**
 * Extract blacklist patterns from .n2 source — returns JSON array
 */
export function extract_blacklist_wasm(source: string): string;

/**
 * Get compiler version info
 */
export function n2c_version(): string;

/**
 * Parse .n2 source and return AST as JSON string
 */
export function parse_n2_wasm(source: string): string;

/**
 * Query .n2 source with SQL — returns formatted table string
 */
export function query_n2_wasm(source: string, sql: string): string;

/**
 * Validate .n2 source — returns JSON with errors/warnings
 */
export function validate_n2_wasm(source: string): string;
