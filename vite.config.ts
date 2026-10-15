// Vite impose un port stable pour que la fenêtre native retrouve toujours son interface.
import { defineConfig } from "vite";
export default defineConfig({
  server: { port: 1420, strictPort: true },
  clearScreen: false,
});
