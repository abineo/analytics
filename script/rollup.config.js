import typescript from '@rollup/plugin-typescript';
import terser from '@rollup/plugin-terser';
import size from 'rollup-plugin-output-size';

/** @type {import('rollup').RollupOptions} */
export default [
	{
		input: 'src/browser.ts',
		output: {
			file: 'dist/browser.min.js',
			name: 'Abineo',
			format: 'iife',
			sourcemap: true,
		},
		plugins: [typescript(), terser(), size()],
	},
];
