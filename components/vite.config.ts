import { fileURLToPath, URL } from 'node:url'
import vue from '@vitejs/plugin-vue'

// vite.config.js
import { resolve } from 'path'
import { defineConfig } from 'vite'
import checker from 'vite-plugin-checker';

export default defineConfig({
  plugins: [
    vue({
      template: {
        compilerOptions: {
          // treat all tags with a dash as custom elements
          isCustomElement: tag => tag.includes('-'),
        },
      },
    }),
    checker({
      vueTsc: true,
    }),
  ],
  build: {
    lib: {
      // Could also be a dictionary or array of multiple entry points
      entry: resolve(__dirname, 'lib/index.js'),
      name: 'LiquidComponents',
      // the proper extensions will be added
      fileName: 'liquid-components',
    },
    /*rollupOptions: {
      // make sure to externalize deps that shouldn't be bundled
      // into your library
      external: ['vue'],
      output: {
        // Provide global variables to use in the UMD build
        // for externalized deps
        globals: {
          vue: 'Vue',
        },
      },
    },*/
  },
})