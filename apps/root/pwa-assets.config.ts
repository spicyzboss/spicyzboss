import { defineConfig } from '@vite-pwa/assets-generator/config';

export default defineConfig({
  headLinkOptions: {
    preset: '2023',
  },
  preset: {
    transparent: {
      sizes: [48, 64, 72, 96, 128, 144, 152, 168, 192, 256, 384, 512, 1024],
      favicons: [[32, 'favicon.ico'], [192, 'favicon-192x192.ico']],
    },
    maskable: {
      sizes: [512],
      padding: 0.45,
    },
    apple: {
      sizes: [180],
      padding: 0.45,
    },
  },
  images: ['public/favicon.svg'],
});
