export type FreeBadge = {
  id: string;
  code: string;
  link: string;
  linkIsExternal: boolean;
  availableUntil?: Date | string;
  image: string;
};
