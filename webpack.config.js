const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const dist = path.resolve(__dirname, 'dist');

module.exports = {
	mode: 'production',
	entry: {
		index: './js/index.ts',
	},
	output: {
		path: dist,
		filename: '[name].js',
	},
	module: {
		rules: [
			{
				test: /\.wasm$/,
				type: 'webassembly/sync',
			},
		],
	},
	ignoreWarnings: [
		// temp, see: https://github.com/rust-random/getrandom/issues/224
		warning =>
			warning.message === 'Critical dependency: the request of a dependency is an expression',
		// warning message caused by wasm-bindgen + webpack
		warning => /__wbg.+free/.test(warning.message),
	],
	experiments: {
		syncWebAssembly: true,
	},
	devServer: {
		static: {
			directory: dist,
		},
	},
	plugins: [
		new CopyPlugin({ patterns: [path.resolve(__dirname, 'static')] }),

		new WasmPackPlugin({
			crateDirectory: __dirname,
		}),
	],
};
