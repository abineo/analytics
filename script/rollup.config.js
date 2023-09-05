import terser from '@rollup/plugin-terser';
import size from 'rollup-plugin-output-size';

/** @type {import('rollup').RollupOptions} */
export default [
	{
		input: 'src/script.js',
		output: {
			file: 'dist/script.min.js',
			name: 'Abineo',
			format: 'iife',
			sourcemap: true,
		},
		plugins: [terser(), size()],
	},
];
