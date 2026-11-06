// tools/inspect.js — clotho_inspect: Read compiled contract contents
const fs = require('fs');
const path = require('path');

function registerInspectTools(server, z, compiler) {
    server.tool(
        'clotho_inspect',
        'Read and display the contents of a compiled .n2 contract file (.n2rs, .n2c, .n2ts, etc).',
        {
            file: z.string().describe('Absolute path to compiled output file (.n2rs, .n2c, .n2c2, .n2go, .n2py, .n2ts)'),
        },
        async ({ file }) => {
            try {
                if (!fs.existsSync(file)) {
                    return { content: [{ type: 'text', text: `File not found: ${file}` }], isError: true };
                }

                const ext = path.extname(file);
                const langMap = {
                    '.n2rs': 'rust', '.n2c': 'c', '.n2c2': 'cpp',
                    '.n2go': 'go', '.n2py': 'python', '.n2ts': 'typescript',
                };

                if (!Object.hasOwn(langMap, ext)) throw new Error('Unsupported compiled-file extension');
                if (fs.statSync(file).size > 8 * 1024 * 1024) throw new Error('Compiled file exceeds 8 MiB');
                const content = fs.readFileSync(file, 'utf-8');
                const summary = [
                    `🧵 **Clotho Inspect**`,
                    `📄 File: ${path.basename(file)}`,
                    `📦 Size: ${content.length} bytes`,
                    ``,
                    '```' + (langMap[ext] || ''),
                    content,
                    '```',
                ];

                return { content: [{ type: 'text', text: summary.join('\n') }] };
            } catch (err) {
                return {
                    content: [{ type: 'text', text: `❌ Inspect error:\n${err.message}` }],
                    isError: true,
                };
            }
        }
    );
}

module.exports = { registerInspectTools };
