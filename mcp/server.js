// n2-clotho MCP Server - Multi-Target AI Contract Compiler
// Supports both stdio and SSE transport modes
// WASM runtime with native fallback.

const { McpServer } = require('@modelcontextprotocol/sdk/server/mcp.js');
const { StdioServerTransport } = require('@modelcontextprotocol/sdk/server/stdio.js');
const { z } = require('zod');
const path = require('path');
const fs = require('fs');
const pkg = require('./package.json');

// ═══════════════════════════════════════════════════════
// Compiler Resolution: WASM (bundled) → Native binary (fallback)
// ═══════════════════════════════════════════════════════

let compiler = null;
const wasmPath = path.resolve(__dirname, 'wasm', 'n2_compiler.js');
const nativeBin = path.resolve(__dirname, '..', 'compiler', 'target', 'release',
    process.platform === 'win32' ? 'n2-compiler.exe' : 'n2-compiler'
);

if (fs.existsSync(wasmPath)) {
    compiler = { type: 'wasm', module: require(wasmPath), bin: null };
} else if (fs.existsSync(nativeBin)) {
    compiler = { type: 'native', module: null, bin: nativeBin };
} else {
    console.error('[n2-clotho] ❌ Neither WASM nor native compiler found!');
    console.error(`  WASM: ${wasmPath}`);
    console.error(`  Native: ${nativeBin}`);
    process.exit(1);
}

function createServer() {
const server = new McpServer({
    name: 'n2-clotho',
    version: pkg.version,
});

// ═══════════════════════════════════════════════════════
// Tool Registration
// ═══════════════════════════════════════════════════════

const { registerCompileTools } = require('./tools/compile');
const { registerBatchTools } = require('./tools/batch');
const { registerValidateTools } = require('./tools/validate');
const { registerBackendTools } = require('./tools/backends');
const { registerInspectTools } = require('./tools/inspect');

registerCompileTools(server, z, compiler);
registerBatchTools(server, z, compiler);
registerValidateTools(server, z, compiler);
registerBackendTools(server, z, compiler);
registerInspectTools(server, z, compiler);
return server;
}

// ═══════════════════════════════════════════════════════
// Boot
// ═══════════════════════════════════════════════════════

async function boot() {
    const mode = process.env.CLOTHO_TRANSPORT || (process.argv.includes('--sse') ? 'sse' : 'stdio');

    if (mode === 'sse') {
        const { SSEServerTransport } = require('@modelcontextprotocol/sdk/server/sse.js');
        const http = require('http');
        const port = Number(process.env.CLOTHO_PORT || '3200');
        if (!Number.isInteger(port) || port < 0 || port > 65535) throw new Error('Invalid CLOTHO_PORT');

        let transport = null;

        const httpServer = http.createServer(async (req, res) => {
            const host = req.headers.host;
            const address = httpServer.address();
            const allowedHosts = [`127.0.0.1:${address.port}`, `localhost:${address.port}`];
            if (!allowedHosts.includes(host) || (req.headers.origin && req.headers.origin !== `http://${host}`)) {
                res.writeHead(403); res.end('Local same-origin requests only'); return;
            }

            if (req.method === 'OPTIONS') {
                res.writeHead(204);
                res.end();
                return;
            }

            try {
            const url = new URL(req.url, `http://${host}`);

            if (url.pathname === '/sse' && req.method === 'GET') {
                if (transport) { res.writeHead(409); res.end('An SSE client is already connected'); return; }
                transport = new SSEServerTransport('/messages', res);
                const active = transport;
                res.on('close', () => { if (transport === active) transport = null; });
                await createServer().connect(transport);
            } else if (url.pathname === '/messages' && req.method === 'POST') {
                if (transport) {
                    await transport.handlePostMessage(req, res);
                } else {
                    res.writeHead(400);
                    res.end('No active SSE connection');
                }
            } else if (url.pathname === '/health') {
                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({
                    status: 'ok',
                    name: 'n2-clotho',
                    version: pkg.version,
                    compiler: compiler.type,
                }));
            } else {
                res.writeHead(404);
                res.end('Not Found');
            }
            } catch (error) {
                if (!res.headersSent) res.writeHead(500);
                res.end('Transport request failed');
                console.error(`[n2-clotho] Transport error: ${error.message}`);
            }
        });

        httpServer.listen(port, '127.0.0.1', () => {
            console.log(`[n2-clotho] SSE server running on port ${httpServer.address().port}`);
        });
    } else {
        const transport = new StdioServerTransport();
        await createServer().connect(transport);
    }
}

boot().catch(err => {
    console.error(`[n2-clotho] Fatal: ${err.message}`);
    process.exit(1);
});
