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
    comment: {
      createComment: (articleId: string) => `/comments/${articleId}/new`,
      deleteComment: (commentId: string) => `/comments/${commentId}/delete`,
    },
  },
  admin: {
    home: "/gremio",
  },
} as const;

Object.freeze(routes);
