import { RequestHandler } from "@builder.io/qwik-city";

export const onRequest: RequestHandler = ({ headers }) => {
  const permissions = [
    'accelerometer=()',
    'camera=()',
    'geolocation=()',
    'gyroscope=()',
    'magnetometer=()',
    'microphone=()',
    'payment=()',
    'usb=()'
  ];

  const securityHeaders = {
    'X-Frame-Options': 'sameorigin',
    'X-XSS-Protection': '1; mode=block',
    'Referrer-Policy': 'same-origin',
    'Permissions-Policy': permissions.join('; '),
  };

  Object.entries(securityHeaders).map(([key, value]) => headers.set(key, value));
};
