import { component$ } from '@builder.io/qwik';
import { useDocumentHead, useLocation } from '@builder.io/qwik-city';

export const RouterHead = component$(() => {
  const head = useDocumentHead();
  const { url } = useLocation();

  return (
    <>
      <title>{head.title}</title>

      <link rel="canonical" href={url.href} />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <link rel="icon" href="/favicon-192x192.ico" sizes="192x192" />
      <link rel="icon" href="/favicon.ico" sizes="32x32" />
      <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
      <link rel="apple-touch-icon" href="/apple-touch-icon-180x180.png" />
      <meta
        name="theme-color"
        media="(prefers-color-scheme: light)"
        content="#fa9370"
      />
      <meta
        name="theme-color"
        media="(prefers-color-scheme: dark)"
        content="#6be4ff"
      />

      {head.meta.map((m, i) => (
        <meta key={i} {...m} />
      ))}

      {head.links.map((l, i) => (
        <link key={i} {...l} />
      ))}

      {head.styles.map((s, i) => (
        <style {...s.props} key={i} dangerouslySetInnerHTML={s.style} />
      ))}
    </>
  );
});
