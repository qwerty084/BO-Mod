import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath, URL } from "node:url";

export default defineConfig(async () => ({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/backend/**"],
    },
  },
  alias: {
    "@": fileURLToPath(new URL("./frontend", import.meta.url)),
  },
}));
