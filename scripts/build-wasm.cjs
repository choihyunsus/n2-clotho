// Remap machine-specific source paths before producing the distributable WASM artifact.
const path = require('node:path');
const os = require('node:os');
const { spawnSync } = require('node:child_process');

const root = path.resolve(__dirname, '..');
const cargo = path.resolve(process.env.CARGO_HOME || path.join(os.homedir(), '.cargo'));
if (process.env.RUSTFLAGS && !process.env.CARGO_ENCODED_RUSTFLAGS) {
  throw new Error('Supply existing Rust options through CARGO_ENCODED_RUSTFLAGS for this build');
}
const flags = process.env.CARGO_ENCODED_RUSTFLAGS?.split('\x1f') ?? [];
for (const [source, target] of [[root, '/n2-clotho'], [cargo, '/cargo']]) {
  for (const variant of new Set([source, source.replaceAll('\\', '/')])) {
    flags.push(`--remap-path-prefix=${variant}=${target}`);
  }
}
const result = spawnSync(process.platform === 'win32' ? 'wasm-pack.exe' : 'wasm-pack', [
  'build', '--target', 'nodejs', '--out-dir', '../mcp/wasm', '--no-pack', '--no-opt', '--features', 'wasm',
], {
  cwd: path.join(root, 'compiler'), stdio: 'inherit', windowsHide: true,
  env: { ...process.env, CARGO_ENCODED_RUSTFLAGS: flags.join('\x1f'),
    CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS || '2',
    RUSTUP_TOOLCHAIN: process.env.RUSTUP_TOOLCHAIN || '1.98.0' },
});
if (result.error) throw result.error;
process.exitCode = result.status ?? 1;
