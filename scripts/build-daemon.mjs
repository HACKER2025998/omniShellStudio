// Le sidecar est compilé sans interface graphique pour respecter le budget du service résident.
import { execFileSync } from "node:child_process";
import { mkdirSync, copyFileSync } from "node:fs";
import path from "node:path";
// Le triple explicite rend le même script utilisable sous Linux, Windows et macOS.
const triple =
  process.env.TAURI_ENV_TARGET_TRIPLE ||
  execFileSync("rustc", ["-vV"], { encoding: "utf8" }).match(
    /^host: (.+)$/m,
  )[1];
// Le mode développement évite une compilation optimisée à chaque première ouverture.
const debug = process.argv.includes("--debug");
const args = [
  "build",
  "--locked",
  "--manifest-path",
  "src-tauri/Cargo.toml",
  "--bin",
  "omnishell-daemon",
  "--no-default-features",
];
// Le build natif utilise le même compilateur que le bureau, sans modifier ses capacités IPC.
if (!debug) args.push("--release");
execFileSync("cargo", args, { stdio: "inherit" });
// Tauri attend le suffixe du triple pour empaqueter un binaire externe.
const extension = triple.includes("windows") ? ".exe" : "";
mkdirSync("src-tauri/binaries", { recursive: true });
copyFileSync(
  path.join(
    "src-tauri/target",
    debug ? "debug" : "release",
    "omnishell-daemon" + extension,
  ),
  path.join("src-tauri/binaries", "omnishell-daemon-" + triple + extension),
);
