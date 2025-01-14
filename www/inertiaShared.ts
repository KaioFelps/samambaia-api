import { ReactElement } from "react";

import { AnnouncementShort } from "./types/announcement";
import { FeaturedUser } from "./types/featuredUsers";
import { Pagination } from "./types/pagination";

export type PageComponent = ReactElement & { default: { layout: Element } };

export const resolveTitle = (title: string | undefined, defaultTitle: string): string => (title
  ? `${defaultTitle} - ${title}`
  : defaultTitle);

export type SharedProps = {
  announcements: { data: AnnouncementShort[]; paginationn: Pagination };
  featuredUsers: { data: FeaturedUser[]; pagination: Pagination };
};
