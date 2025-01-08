import { ReactElement } from "react";

export type PageComponent = ReactElement & { default: { layout: Element } };

export const resolveTitle = (title: string | undefined, defaultTitle: string): string => (title
  ? `${defaultTitle} - ${title}`
  : defaultTitle);
