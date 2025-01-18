import { User } from "./user";

export const Permission = {
  UpdateUser: "UpdateUser",
  ChangeUserPassword: "ChangeUserPassword",
  CreateArticle: "CreateArticle",
  UpdateArticle: "UpdateArticle",
  ApproveArticle: "ApproveArticle",
  DisapproveArticle: "DisapproveArticle",
  SeeUnapprovedArticle: "SeeUnapprovedArticle",
  ChangeArticleAuthor: "ChangeArticleAuthor",
  DeleteArticle: "DeleteArticle",
  InactivateComment: "InactivateComment",
  DeleteComment: "DeleteComment",
  SolveReport: "SolveReport",
  DeleteReport: "DeleteReport",
  CreateNewTeamRole: "CreateNewTeamRole",
  UpdateTeamRole: "UpdateTeamRole",
  DeleteTeamRole: "DeleteTeamRole",
  CreateTeamUser: "CreateTeamUser",
  UpdateTeamUser: "UpdateTeamUser",
  DeleteTeamUser: "DeleteTeamUser",
  CreateArticleTag: "CreateArticleTag",
  UpdateArticleTag: "UpdateArticleTag",
  DeleteArticleTag: "DeleteArticleTag",
  CreateFreeBadge: "CreateFreeBadge",
  UpdateFreeBadge: "UpdateFreeBadge",
  DeleteFreeBadge: "DeleteFreeBadge",
  CreateAnnouncement: "CreateAnnouncement",
  UpdateAnnouncement: "UpdateAnnouncement",
  DeleteAnnouncement: "DeleteAnnouncement",
} as const;

export const Role = {
  Admin: "Admin",
  Ceo: "Ceo",
  Coord: "Coord",
  Editor: "Editor",
  Principal: "Principal",
  User: "User",
  Writer: "Writer",
} as const;

Object.freeze(Role);
Object.freeze(Permission);

export type TPermission = keyof typeof Permission;
export type TRole = keyof typeof Role;

export type Auth = {
  user: User;
  permissions: TPermission[];
};
