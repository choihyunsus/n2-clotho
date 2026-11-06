// Preserve validation failures in the MCP error flag and report.
const { readSource, runNative, errorResult } = require('./runtime');

function registerValidateTools(server, z, compiler) {
    server.tool('clotho_validate', 'Validate .n2 syntax, declarations and state-machine integrity.', {
        source: z.string().describe('Path to a .n2 file or inline source'),
    }, async ({ source }) => {
        try {
            const text = readSource(source);
            if (compiler.type === 'native') {
                return { content: [{ type: 'text', text: runNative(compiler, 'validate', text) }] };
            }
            const report = JSON.parse(compiler.module.validate_n2_wasm(text));
            return { content: [{ type: 'text', text: JSON.stringify(report) }],
                isError: report.errorCount > 0 || report.integrityViolations.length > 0 };
        } catch (error) { return errorResult(error); }
    });
}
module.exports = { registerValidateTools };
