import { PageResolver } from "@inertiajs/core/types";
import type { ReactElement } from "react";

import { AdminLayout } from "@/layouts/admin";
import { PublicLayout } from "@/layouts/public";
import { AnnouncementShort } from "@/types/announcement";
import { Auth } from "@/types/auth";
import { FeaturedUser } from "@/types/featured-users";
import { Pagination } from "@/types/pagination";

type PageComponent = ReactElement & {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  default: { layout: (_page: any) => ReactElement };
};

export const resolveTitle = (title: string | undefined, defaultTitle: string): string => (title
  ? `${defaultTitle} - ${title}`
  : defaultTitle);

export const pageResolver: PageResolver = (name) => {
  const pages = import.meta.glob("../pages/**/*.tsx", { eager: true });
  const page = pages[`../pages/${name}.tsx`] as PageComponent;

  if (!page) throw new Error(`Não foi possível encontrar a página ${name}.`);

  const isAdmin = name.startsWith("admin/");

  page.default.layout ??= (page) =>
    isAdmin
      ? <AdminLayout props={page.props}>{page}</AdminLayout>
      : <PublicLayout>{page}</PublicLayout>;

  return page;
};

declare module "@inertiajs/core/types" {
  export interface PageProps {
    auth?: Auth;
    announcements: { data: AnnouncementShort[]; paginationn: Pagination };
    featuredUsers: { data: FeaturedUser[]; pagination: Pagination };
    flash: Record<string, string>;
  }
}
