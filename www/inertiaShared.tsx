import type { PageProps as DefaultPageProps } from "@inertiajs/core/types";
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

declare module "@inertiajs/core/types" {
  export interface PageProps extends DefaultPageProps {
    auth?: Auth;
    announcements: { data: AnnouncementShort[]; paginationn: Pagination };
    featuredUsers: { data: FeaturedUser[]; pagination: Pagination };
    flash: Record<string, string>;
  }
}
