// tools/batch.js — clotho_batch: Compile .n2 source to ALL target languages at once
const { execFileSync } = require('child_process');
const fs = require('fs');
const path = require('path');

function registerBatchTools(server, z, compilerBin) {
    server.tool(
        'clotho_batch',
        'Compile a .n2 AI behavioral contract to ALL 6 target languages at once. Generates .n2rs, .n2c, .n2c2, .n2go, .n2py, .n2ts files.',
        {
            source: z.string().describe('Absolute path to the .n2 source file'),
            outputDir: z.string().optional()
                .describe('Optional output directory (default: same directory as source)'),
        },
        async ({ source, outputDir }) => {
            try {
                if (!fs.existsSync(source)) {
                    return { content: [{ type: 'text', text: `❌ Source file not found: ${source}` }] };
                }

                const result = execFileSync(compilerBin, ['compile', source, 'all'], {
                    encoding: 'utf-8',
                    timeout: 60000,
                });

                // Collect results
                const extMap = {
                    rust: '.n2rs', c: '.n2c', cpp: '.n2c2',
                    go: '.n2go', python: '.n2py', ts: '.n2ts'
                };
                const basePath = source.replace(/\.n2$/, '');
                const files = [];

                for (const [lang, ext] of Object.entries(extMap)) {
                    const outPath = basePath + ext;
                    if (fs.existsSync(outPath)) {
                        const stat = fs.statSync(outPath);
                        files.push({ lang, ext, path: outPath, size: stat.size });

                        // Move to outputDir if specified
                        if (outputDir) {
                            if (!fs.existsSync(outputDir)) {
                                fs.mkdirSync(outputDir, { recursive: true });
                            }
                            const destPath = path.join(outputDir, path.basename(outPath));
                            fs.copyFileSync(outPath, destPath);
                            fs.unlinkSync(outPath);
                            files[files.length - 1].path = destPath;
                        }
                    }
                }

                const summary = [
                    `🧵 **Clotho Batch Compile** — All targets`,
                    `📄 Source: ${path.basename(source)}`,
                    `📊 Results: ${files.length}/6 targets compiled`,
                    ``,
                    ...files.map(f => `  ✅ ${f.lang.padEnd(8)} → ${path.basename(f.path)} (${f.size} bytes)`),
                    ``,
                    `Total: ${files.reduce((sum, f) => sum + f.size, 0)} bytes across ${files.length} files`,
                ];

                return { content: [{ type: 'text', text: summary.join('\n') }] };
            } catch (err) {
                const stderr = err.stderr ? err.stderr.toString() : err.message;
                return {
                    content: [{ type: 'text', text: `❌ Batch compilation error:\n${stderr}` }],
                    isError: true,
                };
            }
        }
    );
}

module.exports = { registerBatchTools };
