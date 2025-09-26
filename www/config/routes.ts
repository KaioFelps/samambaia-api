export const routes = {
  tools: {
    imager: "/tools/imager",
  },
  web: {
    home: "/",
    article: {
      base: "/article",
      view: (slug: string): string => `/article/${slug}`,
    },
  },
  admin: {
    home: "/gremio",
  },
} as const;

Object.freeze(routes);
