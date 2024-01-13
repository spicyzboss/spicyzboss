import type { Config } from 'tailwindcss';

import { join } from 'path';
import { fontFamily } from 'tailwindcss/defaultTheme';
import { addDynamicIconSelectors } from '@iconify/tailwind';

export default {
  content: [join(__dirname, 'src/**/*.{js,ts,jsx,tsx,mdx}')],
  darkMode: ['class', '[data-mode="dark"]'],
  theme: {
    fontFamily: {
      sans: ['Inter', 'Noto Sans Thai Looped', ...fontFamily.sans],
    },
    extend: {
      colors: {
        primary: {
          DEFAULT: 'rgb(var(--primary) / <alpha-value>)',
          on: 'rgb(var(--on-primary) / <alpha-value>)',
          container: {
            DEFAULT: 'rgb(var(--primary-container) / <alpha-value>)',
            on: 'rgb(var(--on-primary-container) / <alpha-value>)',
          },
          inverse: 'rgb(var(--inverse-primary) / <alpha-value>)',
        },
        secondary: {
          DEFAULT: 'rgb(var(--secondary) / <alpha-value>)',
          on: 'rgb(var(--on-secondary) / <alpha-value>)',
          container: {
            DEFAULT: 'rgb(var(--secondary-container) / <alpha-value>)',
            on: 'rgb(var(--on-secondary-container) / <alpha-value>)',
          },
        },
        tertiary: {
          DEFAULT: 'rgb(var(--tertiary) / <alpha-value>)',
          on: 'rgb(var(--on-tertiary) / <alpha-value>)',
          container: {
            DEFAULT: 'rgb(var(--tertiary-container) / <alpha-value>)',
            on: 'rgb(var(--on-tertiary-container) / <alpha-value>)',
          },
        },
        error: {
          DEFAULT: 'rgb(var(--error) / <alpha-value>)',
          on: 'rgb(var(--on-error) / <alpha-value>)',
          container: {
            DEFAULT: 'rgb(var(--error-container) / <alpha-value>)',
            on: 'rgb(var(--on-error-container) / <alpha-value>)',
          },
        },
        outline: {
          DEFAULT: 'rgb(var(--outline) / <alpha-value>)',
          variant: 'rgb(var(--outline-variant) / <alpha-value>)',
        },
        shadow: 'rgb(var(--shadow) / <alpha-value>)',
        scrim: 'rgb(var(--scrim) / <alpha-value>)',
        background: {
          DEFAULT: 'rgb(var(--background) / <alpha-value>)',
          on: 'rgb(var(--on-background) / <alpha-value>)',
        },
        surface: {
          DEFAULT: 'rgb(var(--surface) / <alpha-value>)',
          on: 'rgb(var(--on-surface) / <alpha-value>)',
          tint: 'rgb(var(--surface-tint) / <alpha-value>)',
          variant: {
            DEFAULT: 'rgb(var(--surface-variant) / <alpha-value>)',
            on: 'rgb(var(--on-surface-variant) / <alpha-value>)',
          },
          inverse: {
            DEFAULT: 'rgb(var(--inverse-surface) / <alpha-value>)',
            on: 'rgb(var(--inverse-on-surface) / <alpha-value>)',
          },
          dim: 'rgb(var(--surface-dim) / <alpha-value>)',
          bright: 'rgb(var(--surface-bright) / <alpha-value>)',
          container: {
            DEFAULT: 'rgb(var(--surface-container) / <alpha-value>)',
            low: 'rgb(var(--surface-container-low) / <alpha-value>)',
            lowest: 'rgb(var(--surface-container-lowest) / <alpha-value>)',
            high: 'rgb(var(--surface-container-high) / <alpha-value>)',
            highest: 'rgb(var(--surface-container-highest) / <alpha-value>)',
          },
        },
      },
    },
  },
  plugins: [addDynamicIconSelectors()],
} satisfies Config;
