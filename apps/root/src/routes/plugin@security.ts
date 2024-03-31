import { RequestHandler } from "@builder.io/qwik-city";

export const onRequest: RequestHandler = ({ headers }) => {
  const securityHeaders = {
    'X-Frame-Options': 'sameorigin',
    'X-XSS-Protection': '1; mode=block',
  };

  Object.entries(securityHeaders).map(([key, value]) => headers.set(key, value));
};
