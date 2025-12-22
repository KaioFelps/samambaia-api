import { Link, usePage } from "@inertiajs/react";
import { clsx } from "clsx";
import { memo, useMemo } from "react";
import { routes } from "@/config/routes";

type Props = {
  title: string;
  slug: string;
};

export const ArticleFeedLink = memo(({ title, slug }: Props) => {
  const url = new URL(usePage().url, "http://localhost"); // base ain't important here
  const isActive = url.pathname.endsWith(slug);

  const LinkOrSpan = useMemo(() => (isActive ? "span" : Link), [isActive]);

  return (
    <LinkOrSpan
      className={clsx(
        "inline-block w-full border-b border-gray-300 text-sm font-normal pb-1",
        isActive ? "text-gray-800 cursor-default" : "text-blue-700 hover:text-blue-500",
      )}
      href={routes.web.article.view(slug)}>
      {title} {!isActive && "»"}
    </LinkOrSpan>
  );
});
