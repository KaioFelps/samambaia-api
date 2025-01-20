import { ReactElement, ReactNode } from "react";

import { PublicLayout } from "./layouts/public";
import { AnnouncementShort } from "./types/announcement";
import { Auth } from "./types/auth";
import { FeaturedUser } from "./types/featuredUsers";
import { Pagination } from "./types/pagination";

export type PageComponent = ReactElement & { default: { layout: Element } };

export const resolveTitle = (title: string | undefined, defaultTitle: string): string => (title
  ? `${defaultTitle} - ${title}`
  : defaultTitle);

export type SharedProps<T extends Record<string, string> = Record<string, string>> = {
  auth?: Auth;
  announcements: { data: AnnouncementShort[]; paginationn: Pagination };
  featuredUsers: { data: FeaturedUser[]; pagination: Pagination };
  flash: T;
};

export function resolvePageLayout(page: PageComponent) {
  const defaultLayout = (page: ReactNode) => <PublicLayout>{page}</PublicLayout>;

  if (!Object.hasOwn(page, "default")) {
    Object.defineProperty(page, "default", { value: { layout: defaultLayout } });
    return page;
  } else if (!Object.hasOwn(page.default, "layout")) {
    Object.defineProperty(page.default, "layout", { value: defaultLayout });
    return page;
  }

  return page;
}
