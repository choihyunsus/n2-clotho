// Public compiler functions retain their synchronous string contracts.
export function compile_n2_wasm(source: string, target: string, compiled_at: string): string;
export function parse_n2_wasm(source: string): string;
export function validate_n2_wasm(source: string): string;
export function query_n2_wasm(source: string, sql: string): string;
export function extract_blacklist_wasm(source: string): string;
export function n2c_version(): string;
export class BoundedRegex {
    constructor(pattern: string, flags?: string);
    test(input: string): boolean;
    free(): void;
}
