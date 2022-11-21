const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const outDir = path.resolve(__dirname, "docs");
const staticDir = path.resolve(__dirname, "static");

module.exports = {
  mode: 'production',
  entry: './src/main.ts',
  output: {
    path: outDir,
    filename: 'bundle.js'
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js']
  },
  experiments: {
    asyncWebAssembly: true
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: staticDir, to: outDir }
      ]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname
    }),
  ]
};