import * as path from 'path';
import webpack from 'webpack';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';

import 'webpack-dev-server';

const config: webpack.Configuration = {
  entry: './wasm-entry.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'public/index.html',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.'), // Rust main entrypoint.
      outDir: path.resolve(__dirname, './pkg'),
    }),
  ],
  mode: 'development',
  experiments: {
    asyncWebAssembly: true,
  },
  devServer: {
    compress: true,
    static: {
      directory: path.join(__dirname),
    },
    port: 5000,
  },
};

export default config;