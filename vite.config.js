import { crx } from "@crxjs/vite-plugin";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import manifest from "./manifest.json";

/** @type {import('vite').UserConfig} */
export default {
  plugins: [crx({ manifest }), wasm(), topLevelAwait()],
  build: {
    rollupOptions: {
      input: ["background.js"],
    },
  },
};
