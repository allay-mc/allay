// @ts-check

import * as path from "node:path";

const rootDir = import.meta.dirname;

const prebuild = process.env.ALLAY_PREBUILD;
const debugMode = process.env.ALLAY_PROFILE == "debug";
if (prebuild === undefined) throw new Error("not in an Allay environment");

/**
 * @type {import('webpack').Configuration}
 */
export default {
  entry: path.resolve(rootDir, "src/BP/scripts/index.ts"),
  mode: "production",
  target: ["es2020"],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  optimization: {
    minimize: !debugMode,
  },
  resolve: {
    enforceExtension: false,
    extensions: [".ts", ".js"],
  },
  output: {
    filename: "index.js",
    path: path.resolve(rootDir, `${process.env.ALLAY_PREBUILD}/BP/scripts`),
  },
  experiments: {
    outputModule: true,
  },
  externalsType: "module",
  externals: {
    "@minecraft/server": "@minecraft/server",
    "@minecraft/server-ui": "@minecraft/server-ui",
    "@minecraft/server-admin": "@minecraft/server-admin",
    "@minecraft/server-gametest": "@minecraft/server-gametest",
    "@minecraft/server-net": "@minecraft/server-net",
    "@minecraft/server-common": "@minecraft/server-common",
    "@minecraft/debug-utilities": "@minecraft/debug-utilities",
  },
};
