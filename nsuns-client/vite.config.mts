import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import compress from "./build/vite-plugin-compression";

export default defineConfig({
  plugins: [
    solidPlugin(),
    compress({
      filter: /\.(js|css|ico)$/i,
    }),
  ],
  server: {
    proxy: {
      "/api": "http://127.0.0.1:8080",
    },
    port: 3000,
  },
  build: {
    target: "esnext",
    reportCompressedSize: false,
  },
});
