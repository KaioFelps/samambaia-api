import type { TRole } from "./auth";

export type Comment = {
  id: string;
  content: string;
  createdAt: Date | string;
  author: {
    nickname: string;
    role: TRole;
  };
};
