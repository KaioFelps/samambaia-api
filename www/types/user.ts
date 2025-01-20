import { TRole } from "./auth";

export type User = {
  id: string;
  nickname: string;
  createdAt: Date | string;
  role: TRole;
};
