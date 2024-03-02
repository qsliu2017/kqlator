import { crx } from "@crxjs/vite-plugin";
import manifest from "./manifest.json";
/** @type {import('vite').UserConfig} */
export default {
  plugins: [crx({ manifest })],
  build: {
    rollupOptions: {
      input: ["background.js"],
    },
  },
};
