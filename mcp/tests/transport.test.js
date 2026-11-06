// Exercise the installed MCP entry point over stdio and loopback SSE.
const { test } = require('node:test');
const assert = require('node:assert/strict');
const { spawn } = require('node:child_process');
const path = require('node:path');
const http = require('node:http');
const { once } = require('node:events');
const { Client } = require('@modelcontextprotocol/sdk/client/index.js');
const { StdioClientTransport } = require('@modelcontextprotocol/sdk/client/stdio.js');
const { SSEClientTransport } = require('@modelcontextprotocol/sdk/client/sse.js');

const entry = path.resolve(__dirname, '../server.js');
const source = '@meta { name: "Probe" enforce: strict } @rule Safe { blacklist: [/blocked/] }';

async function checkTools(client) {
    const list = await client.listTools();
    assert.equal(list.tools.length, 5);
    const schema = list.tools.find(tool => tool.name === 'clotho_compile').inputSchema;
    assert.equal(schema.properties.source.type, 'string');
    assert.deepEqual(schema.properties.target.enum, ['rust', 'c', 'cpp', 'go', 'python', 'ts']);
    assert.deepEqual(schema.required, ['source', 'target']);
    const malformed = await client.callTool({ name: 'clotho_compile', arguments: { source: [], target: 'python' } });
    assert.equal(malformed.isError, true);
    const missingSource = await client.callTool({ name: 'clotho_compile', arguments: { target: 'python' } });
    assert.equal(missingSource.isError, true);
    const badTarget = await client.callTool({ name: 'clotho_compile', arguments: { source, target: 'shell' } });
    assert.equal(badTarget.isError, true);
    const compiled = await client.callTool({ name: 'clotho_compile', arguments: { source, target: 'python' } });
    assert.ok(!compiled.isError);
    assert.match(JSON.parse(compiled.content[0].text).code, /class SafeRule/);
    const batch = await client.callTool({ name: 'clotho_batch', arguments: { source } });
    assert.ok(!batch.isError);
    assert.equal(JSON.parse(batch.content[0].text).outputs.length, 6);
    const valid = await client.callTool({ name: 'clotho_validate', arguments: { source } });
    assert.ok(!valid.isError);
    const invalid = await client.callTool({ name: 'clotho_validate', arguments: { source: '@meta {' } });
    assert.equal(invalid.isError, true);
    const missing = await client.callTool({ name: 'clotho_inspect', arguments: { file: 'missing.n2ts' } });
    assert.equal(missing.isError, true);
    const backends = await client.callTool({ name: 'clotho_backends', arguments: {} });
    assert.match(backends.content[0].text, /6 targets/);
}

test('stdio entry point exposes five working tools', { timeout: 15000 }, async () => {
    const client = new Client({ name: 'release-test', version: '1' });
    try {
        await client.connect(new StdioClientTransport({ command: process.execPath,
            args: [entry], env: { ...process.env, CLOTHO_TRANSPORT: 'stdio' }, stderr: 'pipe' }));
        await checkTools(client);
    } finally { await client.close(); }
});

test('SSE health, access boundaries and sequential reconnect work', { timeout: 20000 }, async () => {
    const child = spawn(process.execPath, [entry, '--sse'], {
        env: { ...process.env, CLOTHO_TRANSPORT: 'sse', CLOTHO_PORT: '0' }, stdio: ['ignore', 'pipe', 'pipe'] });
    const closed = once(child, 'exit');
    try {
        const port = await new Promise((resolve, reject) => {
            const timer = setTimeout(() => reject(new Error('SSE startup timed out')), 10000);
            let output = '';
            child.stdout.on('data', chunk => {
                output += chunk.toString();
                const match = output.match(/running on port (\d+)/);
                if (match) { clearTimeout(timer); resolve(Number(match[1])); }
            });
            child.once('exit', () => { clearTimeout(timer); reject(new Error('SSE exited before startup')); });
            child.once('error', error => { clearTimeout(timer); reject(error); });
        });
        const url = `http://127.0.0.1:${port}`;
        assert.equal((await (await fetch(`${url}/health`)).json()).compiler, 'wasm');
        assert.equal((await fetch(`${url}/health`, { headers: { Origin: 'https://example.com' } })).status, 403);
        const foreignHost = await new Promise((resolve, reject) => {
            http.get(`${url}/health`, { headers: { Host: 'example.com' } }, response => {
                response.resume(); resolve(response.statusCode);
            }).on('error', reject);
        });
        assert.equal(foreignHost, 403);
        for (let attempt = 0; attempt < 2; attempt += 1) {
            const client = new Client({ name: 'sse-release-test', version: '1' });
            try {
                await client.connect(new SSEClientTransport(new URL(`${url}/sse`)));
                assert.equal((await fetch(`${url}/sse`)).status, 409);
                await checkTools(client);
            } finally { await client.close(); }
            await new Promise(resolve => setTimeout(resolve, 100));
        }
    } finally { child.kill(); await closed; }
});
