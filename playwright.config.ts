// Les tests utilisent Chrome installé ou CHROME_BIN, sans télécharger un navigateur caché.
import { defineConfig } from "@playwright/test";
// Le serveur Vite est temporaire et n'expose aucun accès au système hôte.
export default defineConfig({
  testDir: "tests/ui",
  workers: 1,
  use: {
    baseURL: "http://127.0.0.1:1420",
    channel: "chrome",
    viewport: { width: 1440, height: 1000 },
    launchOptions: {
      executablePath: process.env.CHROME_BIN,
      args: ["--no-sandbox"],
    },
  },
  webServer: {
    command: "npm run dev",
    url: "http://127.0.0.1:1420",
    reuseExistingServer: !process.env.CI,
  },
  reporter: "list",
});
