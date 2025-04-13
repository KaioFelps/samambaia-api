import { ArticleTag } from "./article-tag";

export type Article = {
  id: string;
  authorId: string;
  coverUrl: string;
  title: string;
  content: string;
  description: string;
  approved: boolean;
  createdAt: Date | string;
  updatedAt?: Date | string;
  slug: string;
  tags: ArticleTag[];
};
