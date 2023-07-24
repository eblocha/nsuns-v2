import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
// import devtools from 'solid-devtools/vite';

export default defineConfig({
  plugins: [
    /* 
    Uncomment the following line to enable solid-devtools.
    For more info see https://github.com/thetarnav/solid-devtools/tree/main/packages/extension#readme
    */
    // devtools(),
    solidPlugin(),
  ],
  server: {
    proxy: {
      "/api": "http://127.0.0.1:8080",
    },
    port: 3000,
  },
  build: {
    target: "esnext",
    outDir: "../dist/assets",
    emptyOutDir: true,
    assetsDir: "."
  },
});
