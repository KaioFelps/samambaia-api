import type { Article } from "@/types/article";

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
      list: "/gremio/noticias",
      create: "/gremio/noticias/nova",
      edit: (articleId: Article["id"]) => `/gremio/noticias/${articleId}/editar`,
      updateChanges: (articleId: Article["id"]) => `/gremio/noticias/${articleId}/atualizar`,
      storeNewArticle: "/gremio/noticias/criar",
      toggleApproved: (articleId: string) => `/gremio/noticias/${articleId}/alterar-aprovado`,
      delete: (articleId: Article["id"]) => `/gremio/noticias/${articleId}/apagar`,
    },
  },
} as const;

Object.freeze(routes);
