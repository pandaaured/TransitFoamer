import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte()],
  build: {
    outDir: '../dist',
    emptyOutDir: true,
  },
  server: {
    proxy: {
      '/schedule': "http://127.0.0.1:8080",
      '/routes': "http://127.0.0.1:8080",
      '/rtlist': "http://127.0.0.1:8080",
      '/stoptimes': "http://127.0.0.1:8080",
    }

  }
})
