const path = require('path');
const fs = require('fs');
const CopyPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = function () {
  const build = path.join(__dirname, 'build');

  try {
    console.log(`Pre build dir`, fs.readdirSync(path.join(__dirname, 'build')));
  } catch (_) {}

  return {
    mode: 'production',
    entry: {
      utils: './js/resourceid.js'
    },
    output: {
      path: build,
      filename: '[name].js'
    },
    devServer: {
      contentBase: build
    },
    plugins: [
      new CopyPlugin([path.resolve(__dirname, 'public'), path.resolve(__dirname, 'package.json')]),
      new WasmPackPlugin({
        extraArgs: `--scope webb-tools --target nodejs`,
        crateDirectory: __dirname,
        outDir: build,
        outName: 'wasm-utils'
      })
    ]
  };
};
