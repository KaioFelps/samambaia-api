import type { ArticleTag } from "./article-tag";

export type ArticlePreview = {
  id: string;
  slug: string;
  title: string;
  author: { nickname: string; id: string };
  coverUrl: string;
  approved: boolean;
  createdAt: Date | string;
  description: string;
  tags: ArticleTag[];
};
