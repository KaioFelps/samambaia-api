import type { PageResolver } from "@inertiajs/core/types";
import type { JSX, ReactElement } from "react";

import { AdminLayout } from "@/layouts/admin";
import { PublicLayout } from "@/layouts/public";
import type { AnnouncementShort } from "@/types/announcement";
import type { Auth } from "@/types/auth";
import type { FeaturedUser } from "@/types/featured-users";
import type { Pagination } from "@/types/pagination";

type PageComponent = ReactElement & {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  default: { layout: (_page: JSX.Element) => ReactElement };
};

export const resolveTitle = (title: string | undefined, defaultTitle: string): string =>
  title ? `${defaultTitle} :: ${title}` : defaultTitle;

export const pageResolver: PageResolver = async (name) => {
  const pages = import.meta.glob("../pages/**/*.tsx", { eager: false });
  const pagePromise = pages[`../pages/${name}.tsx`];

  if (!pagePromise) throw new Error(`Não foi possível encontrar a página ${name}.`);

  const page = (await pagePromise()) as PageComponent;
  const isAdmin = name.startsWith("admin/");

  page.default.layout ??= (page) =>
    isAdmin ? (
      <AdminLayout props={page.props}>{page}</AdminLayout>
    ) : (
      <PublicLayout>{page}</PublicLayout>
    );

  return page;
};

declare module "@inertiajs/core/types" {
  export interface PageProps {
    auth?: Auth;
    announcements: { data: AnnouncementShort[]; pagination: Pagination };
    featuredUsers: { data: FeaturedUser[]; pagination: Pagination };
    flash: Record<string, string>;
    verificationMotto: string;
  }
}
