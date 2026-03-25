// Compilation output verification tests — validates generated code correctness
const { describe, it } = require('node:test');
const assert = require('node:assert/strict');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const COMPILER_DIR = path.resolve(__dirname, '../../compiler');
const EXAMPLES_DIR = path.join(COMPILER_DIR, 'examples');
const COMPILER_BIN = path.join(COMPILER_DIR, 'target/release/n2-compiler.exe');

// Helper: compile a .n2 file to a specific target
function compile(filename, target = 'all') {
  const filepath = path.join(EXAMPLES_DIR, filename);
  execSync(
    `cargo run --release -- compile "${filepath}" --target=${target}`,
    { cwd: COMPILER_DIR, stdio: 'pipe' }
  );
}

// Helper: read generated output file
function readOutput(basename, ext) {
  return fs.readFileSync(path.join(EXAMPLES_DIR, `${basename}${ext}`), 'utf-8');
}

describe('Compiler Build', () => {
  it('should build successfully', () => {
    const output = execSync('cargo build --release 2>&1', {
      cwd: COMPILER_DIR,
      encoding: 'utf-8',
    });
    assert.ok(!output.includes('error['), 'Build should have no errors');
  });

  it('should report version 3.0.0', () => {
    const output = execSync('cargo run --release -- backends', {
      cwd: COMPILER_DIR,
      encoding: 'utf-8',
    });
    assert.ok(output.includes('v3.0.0'), 'Version should be 3.0.0');
  });
});

describe('CLI --target parsing', () => {
  it('--target=rust (joined) compiles only rust', () => {
    compile('soul-boot.n2', 'rust');
    const out = readOutput('soul-boot', '.n2rs');
    assert.ok(out.includes('Rust target'), 'Should be Rust output');
  });

  it('--target rust (space-separated) compiles only rust', () => {
    const filepath = path.join(EXAMPLES_DIR, 'soul-boot.n2');
    const output = execSync(
      `cargo run --release -- compile "${filepath}" --target rust`,
      { cwd: COMPILER_DIR, encoding: 'utf-8' }
    );
    assert.ok(output.includes('rust →'), 'Should compile rust target');
    assert.ok(!output.includes('Compiling to all targets'), 'Should NOT compile all targets');
  });

  it('--target=all compiles all 6 targets', () => {
    const filepath = path.join(EXAMPLES_DIR, 'soul-boot.n2');
    const output = execSync(
      `cargo run --release -- compile "${filepath}" --target=all`,
      { cwd: COMPILER_DIR, encoding: 'utf-8' }
    );
    assert.ok(output.includes('6 success'), 'Should compile all 6 targets');
  });
});

describe('C backend — #endif guard', () => {
  it('generated .n2c should have closing #endif', () => {
    compile('soul-boot.n2', 'c');
    const code = readOutput('soul-boot', '.n2c');
    assert.ok(code.includes('#ifndef N2_CONTRACT_H'), 'Should have #ifndef guard');
    assert.ok(code.includes('#endif'), 'Should have closing #endif');
  });
});

describe('Go backend — conditional imports', () => {
  it('soul-boot (no contract) should NOT import fmt', () => {
    compile('soul-boot.n2', 'go');
    const code = readOutput('soul-boot', '.n2go');
    assert.ok(!code.includes('"fmt"'), 'Should NOT import fmt when no contract');
    assert.ok(code.includes('"strings"'), 'Should import strings for blacklist rule');
  });

  it('auto-build (has contract + rule) should import both fmt and strings', () => {
    compile('auto-build.n2', 'go');
    const code = readOutput('auto-build', '.n2go');
    assert.ok(code.includes('"fmt"'), 'Should import fmt for contract');
    assert.ok(code.includes('"strings"'), 'Should import strings for rule');
  });
});

describe('Blacklist pattern cleaning', () => {
  // Tests that /DROP TABLE/i is correctly cleaned to "DROP TABLE" (no trailing /)
  const TARGETS = ['n2rs', 'n2c', 'n2c2', 'n2go', 'n2py', 'n2ts'];

  it('compile soul-boot to all targets', () => {
    compile('soul-boot.n2', 'all');
  });

  for (const ext of TARGETS) {
    it(`${ext}: DROP TABLE pattern should NOT have trailing /`, () => {
      const code = readOutput('soul-boot', `.${ext}`);
      assert.ok(code.includes('DROP TABLE'), `Should contain DROP TABLE in .${ext}`);
      assert.ok(!code.includes('DROP TABLE/'), `Should NOT contain DROP TABLE/ in .${ext}`);
    });
  }
});

describe('ISO 8601 timestamp', () => {
  it('compiled output should have real ISO 8601 timestamp', () => {
    compile('soul-boot.n2', 'rust');
    const code = readOutput('soul-boot', '.n2rs');
    // Should match YYYY-MM-DDTHH:MM:SSZ pattern
    const timestampMatch = code.match(/Compiled:\s+(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z)/);
    assert.ok(timestampMatch, 'Should have ISO 8601 timestamp');
    assert.ok(!code.includes('compiled-by-n2c'), 'Should NOT have fake timestamp');
  });
});

describe('Version consistency', () => {
  it('all outputs should reference n2c v3.0.0', () => {
    compile('soul-boot.n2', 'all');
    for (const ext of ['.n2rs', '.n2c', '.n2c2', '.n2go', '.n2py', '.n2ts']) {
      const code = readOutput('soul-boot', ext);
      assert.ok(code.includes('n2c v3.0.0'), `${ext} should reference n2c v3.0.0`);
    }
  });
});
