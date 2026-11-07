// Verify named CommonJS exports and declarations from an ESM TypeScript consumer.
import { BoundedRegex, parse_n2_wasm, n2c_version } from 'n2-clotho';

const predicate = new BoundedRegex('allowed', 'i');
const matched: boolean = predicate.test('ALLOWED');
const parsed: string = parse_n2_wasm('@meta { name: "ESM" }');
const version: string = n2c_version();
predicate.free();
void matched;
void parsed;
void version;
