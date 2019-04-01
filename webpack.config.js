const path = require('path');

module.exports = {
	entry: './src/index.js',
	output: {
		path: path.resolve(__dirname),
		filename: 'dist/bundle.js',
		chunkFilename: 'dist/chunk.bundle.js',
		webassemblyModuleFilename: 'dist/bundle.wasm'
	},
	mode: 'production',
	optimization: {
		namedModules: true,
		namedChunks: true,
	}
};