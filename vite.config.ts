// Vite impose un port stable pour que la fenêtre native retrouve toujours son interface.
import { defineConfig } from "vite";
export default defineConfig({
  server: { port: 1420, strictPort: true },
  clearScreen: false,
  build: {
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (
            id.includes("@xyflow") ||
            id.includes("@dagrejs") ||
            id.includes("html-to-image")
          )
            return "graph-workspace";
        },
      },
    },
  }, // Les outils de graphes sont regroupés séparément du coffre quotidien.
});
