import { router, usePage } from "@inertiajs/react";
import { useEffect, useState } from "react";
import { routes } from "@/config/routes";
import { PaginationPolitics } from "@/core/politics/pagination-politics";

export type TextFilter = "title" | "author";

const TAG_KEY = "includingTags[]";
const PAGE_KEY = "page";
const TEXT_KEYS: TextFilter[] = ["title", "author"];

export type ArchiveFilters = ReturnType<typeof useArchiveFilters>;

export function useArchiveFilters() {
  const pageUrl = usePage().url;

  const searchParams = new URLSearchParams(
    pageUrl.includes("?") ? pageUrl.slice(pageUrl.indexOf("?") + 1) : "",
  );

  const [filter, setFilter] = useState<TextFilter>("title");
  const [query, setQuery] = useState("");
  const [processing, setProcessing] = useState(false);

  useEffect(() => {
    const search = Object.entries(PaginationPolitics.getQueryObjectFromUrl(pageUrl)).find(([key]) =>
      TEXT_KEYS.includes(key as TextFilter),
    );

    if (search) {
      setFilter(search[0] as TextFilter);
      setQuery(search[1] as string);
    }
  }, [pageUrl]);

  function visit(params: URLSearchParams) {
    router.visit(routes.web.article.archive(params), {
      only: ["feed", "queryErrors"],
      preserveScroll: true,
      onStart: () => setProcessing(true),
      onFinish: () => setProcessing(false),
    });
  }

  function submitTextFilter() {
    const params = new URLSearchParams(searchParams);
    TEXT_KEYS.forEach((key) => {
      params.delete(key);
    });
    params.delete(PAGE_KEY);

    const trimmed = query.trim();
    if (trimmed) params.set(filter, trimmed);

    visit(params);
  }

  function toggleTag(tagId: number) {
    const id = tagId.toString();
    const params = new URLSearchParams(searchParams);

    if (params.has(TAG_KEY, id)) params.delete(TAG_KEY, id);
    else params.append(TAG_KEY, id);

    params.delete(PAGE_KEY);
    visit(params);
  }

  function goToPage(target: number) {
    const params = new URLSearchParams(searchParams);
    params.set(PAGE_KEY, target.toString());
    visit(params);
  }

  return {
    filter,
    setFilter,
    query,
    setQuery,
    processing,
    selectedTagIds: searchParams.getAll(TAG_KEY),
    submitTextFilter,
    toggleTag,
    goToPage,
  };
}
