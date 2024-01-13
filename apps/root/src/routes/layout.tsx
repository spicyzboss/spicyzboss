import { component$, Slot, $, useOnWindow } from '@builder.io/qwik';
import type { RequestHandler } from '@builder.io/qwik-city';

export const onGet: RequestHandler = async ({ cacheControl }) => {
  const YearInSeconds = 60 * 60 * 24 * 365;

  const cacheControlOptions = {
    public: true,
    maxAge: 5,
    sMaxAge: 10,
    staleWhileRevalidate: YearInSeconds,
  };

  cacheControl(cacheControlOptions);

  cacheControl(cacheControlOptions, 'CDN-Cache-Control');
};

export default component$(() => {
  useOnWindow(
    'load',
    $(() => {
      function setTheme(event: MediaQueryList | MediaQueryListEvent) {
        if (
          localStorage.theme === 'dark' ||
          (!('theme' in localStorage) && event.matches)
        ) {
          console.debug('data-mode: dark');
          document.documentElement.dataset.mode = 'dark';
        } else {
          console.debug('data-mode: light');
          document.documentElement.dataset.mode = 'light';
        }
      }

      const darkModeMediaQuery = window.matchMedia(
        '(prefers-color-scheme: dark)',
      );
      console.debug('listener: data-mode enabled');
      setTheme(darkModeMediaQuery);

      darkModeMediaQuery.onchange = setTheme;
    }),
  );

  return <Slot />;
});
