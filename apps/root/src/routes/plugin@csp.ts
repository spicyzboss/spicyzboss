import type { RequestHandler } from '@builder.io/qwik-city';
import { isDev } from '@builder.io/qwik/build';

export const onRequest: RequestHandler = ({ request, sharedMap, headers }) => {
  if (isDev) return;

  const nonce = request.headers.get('cf-ray') || Date.now().toString(36);
  sharedMap.set('@nonce', nonce);

  const csp = [
    `default-src 'self' 'unsafe-inline'`,
    "font-src 'self' https://fonts.gstatic.com",
    "img-src 'self' 'unsafe-inline' data:",
    `script-src 'self' 'unsafe-inline' https: 'nonce-${nonce}' 'strict-dynamic'`,
    "style-src 'self' 'unsafe-inline' https://fonts.googleapis.com",
    `frame-src 'self' 'nonce-${nonce}'`,
    "object-src 'none'",
    "base-uri 'self'",
  ];

  headers.set('Content-Security-Policy', csp.join('; '));
};
