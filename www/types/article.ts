export type Article = {
  id: string;
  author: {
    id: string;
    nickname: string;
  };
  coverUrl: string;
  title: string;
  content: string;
  description: string;
  approved: boolean;
  createdAt: Date | string;
  updatedAt?: Date | string;
  slug: string;
};
