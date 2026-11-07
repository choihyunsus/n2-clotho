// Type-check every public compiler operation from a clean installed consumer.
import {
    BoundedRegex, compile_n2_wasm, parse_n2_wasm, validate_n2_wasm,
    query_n2_wasm, extract_blacklist_wasm, n2c_version,
} from 'n2-clotho';

const source = '@meta { name: "Consumer" enforce: strict }';
const outputs: string[] = [
    compile_n2_wasm(source, 'ts', new Date().toISOString()),
    parse_n2_wasm(source), validate_n2_wasm(source),
    query_n2_wasm(source, 'SELECT * FROM contracts'),
    extract_blacklist_wasm(source), n2c_version(),
];
const predicate = new BoundedRegex('^\\w+$');
const matched: boolean = predicate.test('한글_123');
predicate.free();
void outputs;
void matched;
