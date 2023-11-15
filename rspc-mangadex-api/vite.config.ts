import { AliasOptions, defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import generouted from '@generouted/react-router/plugin'
import tsConfig from "./tsconfig.json";
import { resolve } from "path";

function generateAliases(): AliasOptions {
    const returns: AliasOptions = {};
    const tsPaths = tsConfig.compilerOptions.paths;
    for (const key in tsPaths) {
        returns[key.replace("/*", "")] = resolve(__dirname, tsPaths[key][0].replace("/*", ""));
    }
    return returns;
}

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react(), generouted()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  resolve: {
    alias: {
      ...generateAliases(),
    }
  }
}));
