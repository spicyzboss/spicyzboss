import { qwikVite } from '@builder.io/qwik/optimizer';
import { qwikCity } from '@builder.io/qwik-city/vite';
import { defineConfig } from 'vite';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';
import { qwikNxVite } from 'qwik-nx/plugins';
import tsconfigPaths from 'vite-tsconfig-paths';

export default defineConfig({
  root: __dirname,
  build: {
    outDir: '../../dist/apps/root',
    reportCompressedSize: true,
    commonjsOptions: {
      transformMixedEsModules: true,
    },
  },
  cacheDir: '../../node_modules/.vite/apps/root',
  plugins: [
    tsconfigPaths({ root: '../../' }),
    qwikNxVite(),
    qwikCity(),
    qwikVite({
      client: {
        outDir: '../../dist/apps/root/client',
      },
      ssr: {
        outDir: '../../dist/apps/root/server',
      },
    }),
    nxViteTsPaths(),
  ],
  server: {
    fs: {
      // Allow serving files from the project root
      allow: ['../../'],
    },
  },
  preview: {
    headers: {
      'Cache-Control': 'public, max-age=600',
    },
  },
  test: {
    reporters: ['default'],
    coverage: {
      reportsDirectory: '../../coverage/apps/root',
      provider: 'v8',
    },
    globals: true,
    cache: {
      dir: '../../node_modules/.vitest',
    },
    environment: 'node',
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
  },
});
