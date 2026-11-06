// Return actual generated source from WASM and native compiler backends.
const { EXTENSIONS, readSource, compileSource, errorResult } = require('./runtime');

function registerCompileTools(server, z, compiler) {
    server.tool('clotho_compile', 'Generate the supported .n2 subset in a selected language.', {
        source: z.string().describe('Path to a .n2 file or inline source'),
        target: z.enum(['rust', 'c', 'cpp', 'go', 'python', 'ts']),
    }, async ({ source, target }) => {
        try {
            const code = compileSource(compiler, readSource(source), target);
            return { content: [{ type: 'text', text: JSON.stringify({ target, extension: EXTENSIONS[target], code }) }] };
        } catch (error) { return errorResult(error); }
    });
}
module.exports = { registerCompileTools };
