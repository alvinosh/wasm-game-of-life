const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/entry.js",
  },
  output: {
    path: dist,
    filename: "bundle.js",
  },
  experiments: {
    asyncWebAssembly: true,
  },

  plugins: [
    new CopyPlugin({
      patterns: [{ from: "static", to: dist }],
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
};
