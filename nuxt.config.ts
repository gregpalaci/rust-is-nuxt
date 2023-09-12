// https://nuxt.com/docs/api/configuration/nuxt-config
import { defineNuxtConfig } from "nuxt/config";

export default defineNuxtConfig({
  modules: ["nuxt-vitest", "./module", "@vite-pwa/nuxt"],
  workbox: {
    //offlineAssets: ["_nuxt/*"],
    offlineStrategy: "CacheFirst",
    cachingExtensions: ["js", "css", "html", "png", "svg", "ico", "mjs"],
    enabled: true,
    navigateFallback: "/",
    globPatterns: ["**/*.{js,css,html,png,svg,ico,mjs}"],
  },
  pwa: {
    client: {
      installPrompt: true,
      // you don't need to include this: only for testing purposes
      // if enabling periodic sync for update use 1 hour or so (periodicSyncForUpdates: 3600)
      periodicSyncForUpdates: 20,
    },
    devOptions: {
      enabled: true,
      suppressWarnings: true,
      navigateFallbackAllowlist: [/^\/$/],
      type: "module",
    },


    registerType: "autoUpdate",
    manifest: {
      name: "Nuxt Vite PWA",
      short_name: "NuxtVitePWA",
      theme_color: "#ffffff",
      icons: [
        {
          src: "pwa-192x192.png",
          sizes: "192x192",
          type: "image/png",
        },
        {
          src: "pwa-512x512.png",
          sizes: "512x512",
          type: "image/png",
        },
        {
          src: "pwa-512x512.png",
          sizes: "512x512",
          type: "image/png",
        },
      ],
    },
  },
  imports: {
    autoImport: true,
  },
  devtools: { enabled: false },

  ssr: false,

  appConfig: {
    // you don't need to include this: only for testing purposes
    buildDate: new Date().toISOString(),
  },

  experimental: {
    payloadExtraction: false,
  },
  nitro: {
    esbuild: {
      options: {
        target: "esnext",
      },
    },
    prerender: {
      routes: ["/", "/about"],
    },
  },

  //buildModules: ["@nuxtjs/pwa"],
});
