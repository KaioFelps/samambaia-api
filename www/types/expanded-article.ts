import type { ArticleTag } from "./article-tag";
import type { Comment } from "./comment";
import type { Paginated } from "./pagination";
import type { User } from "./user";

export type ExpandedArticle = {
  id: string;
  coverUrl: string;
  title: string;
  content: string;
  description: string;
  approved: boolean;
  creatdAt: Date | string;
  updatedAt: Date | string;
  slug: string;
  author: User;
  comments: Paginated<Comment>;
  tags: Array<ArticleTag>;
};
