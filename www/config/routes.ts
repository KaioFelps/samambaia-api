export const routes = {
  tools: {
    imager: "/tools/imager",
  },
  web: {
    home: "/",
    article: {
      base: "/article",
      view: (slug: string): string => `/noticias/${slug}`,
    },
    comment: {
      createComment: (articleId: string) => `/comments/${articleId}/new`,
      deleteComment: (commentId: string) => `/comments/${commentId}/delete`,
      report: {
        create: (commentId: string) => `/comment_reports/${commentId}/new`,
      },
    },
  },
  admin: {
    home: "/gremio",
    articles: {
      create: "/gremio/noticias/nova",
    },
  },
} as const;

Object.freeze(routes);
