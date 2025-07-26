import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig, loadEnv } from "vite";
import dotenv from "dotenv";
import { nodePolyfills } from "vite-plugin-node-polyfills";
import { svelteTesting } from "@testing-library/svelte/vite";

dotenv.config();

export default defineConfig(({ mode }) => {
  // Load env file based on `mode` in the current working directory.
  // Set the third parameter to '' to load all env regardless of the `VITE_` prefix.
  const env = loadEnv(mode, process.cwd(), '');

  return {
    plugins: [sveltekit(), svelteTesting(), nodePolyfills()],

    server: {
      proxy: {
        "/api": {
          target:
            env.DFX_NETWORK === "ic"
              ? "https://ic0.app"
              : "http://localhost:8080",
          changeOrigin: true,
        },
      },
    },

    define: {
      // Map dfx generated variables to VITE_ prefixed ones
      "import.meta.env.VITE_DFX_NETWORK": JSON.stringify(process.env.DFX_NETWORK || "local"),
      "import.meta.env.VITE_CANISTER_ID_BACKEND": JSON.stringify(process.env.CANISTER_ID_BACKEND),
      "import.meta.env.VITE_CANISTER_ID_II": JSON.stringify(process.env.CANISTER_ID_II),
    },

    preview: {
      port: 5173,
      host: "0.0.0.0",
    },

    build: {
      target: "esnext",
      sourcemap: true,
    },

    test: {
      environment: "jsdom",
    },
  };
});
