// Return actual generated outputs and per-target failures.
const { EXTENSIONS, readSource, compileSource, errorResult } = require('./runtime');

function registerBatchTools(server, z, compiler) {
    server.tool('clotho_batch', 'Generate all six targets and report each result or error.', {
        source: z.string().describe('Path to a .n2 file or inline source'),
    }, async ({ source }) => {
        try {
            const text = readSource(source);
            const outputs = Object.entries(EXTENSIONS).map(([target, extension]) => {
                try { return { target, extension, code: compileSource(compiler, text, target) }; }
                catch (error) { return { target, extension, error: errorResult(error).content[0].text }; }
            });
            return { content: [{ type: 'text', text: JSON.stringify({ outputs }) }],
                isError: outputs.some(output => Object.hasOwn(output, 'error')) };
        } catch (error) { return errorResult(error); }
    });
}
module.exports = { registerBatchTools };
