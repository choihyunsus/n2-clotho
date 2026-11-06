// tools/backends.js — clotho_backends: List supported compilation targets

const BACKENDS = [
    { name: 'Rust',       target: 'rust',   ext: '.n2rs', use: 'High-performance native runtime' },
    { name: 'C',          target: 'c',      ext: '.n2c',  use: 'Embedded/IoT/System' },
    { name: 'C++',        target: 'cpp',    ext: '.n2c2', use: 'Game engines/HPC' },
    { name: 'Go',         target: 'go',     ext: '.n2go', use: 'Cloud/Microservices' },
    { name: 'Python',     target: 'python', ext: '.n2py', use: 'AI/ML pipelines' },
    { name: 'TypeScript', target: 'ts',     ext: '.n2ts', use: 'Web/Node.js/MCP' },
];

function registerBackendTools(server, z, compiler) {
    server.tool(
        'clotho_backends',
        'List all supported Clotho compilation target languages and their file extensions.',
        {},
        async () => {
            const header = `🧵 **Clotho Supported Backends** (${compiler.type} runtime)\n`;
            const table = BACKENDS.map(b =>
                `  ${b.name.padEnd(12)} → ${b.ext.padEnd(6)} | ${b.use}`
            ).join('\n');

            return {
                content: [{
                    type: 'text',
                    text: `${header}\n${table}\n\nTotal: ${BACKENDS.length} targets`,
                }],
            };
        }
    );
}

module.exports = { registerBackendTools };
