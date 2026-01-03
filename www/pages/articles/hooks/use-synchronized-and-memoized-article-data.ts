import { useRef } from "react";
import type { ArticleTag } from "@/types/article-tag";
import type { ExpandedArticle } from "@/types/expanded-article";
import { contentsAreEquivalent, decodeQuotes } from "@/utils/quotes";

function nullableContentsAreEquivalent(a: string | null, b: string | null) {
  return contentsAreEquivalent(a ?? undefined, b ?? undefined);
}

function arraysAreEquivalent<T>(_a: T[], _b: T[]) {
  if (_a.length !== _b.length) return false;

  const a = _a.map((element) => JSON.stringify(element));
  const b = _b.map((element) => JSON.stringify(element));

  if (!Object.hasOwn(Set.prototype, "isSubsetOf")) {
    return a.every((element) => b.includes(element));
  }

  const setA = new Set(a);
  const setB = new Set(b);

  // its faster to make the subset check using hash sets than simple vectors
  return setA.size === setB.size && setA.isSubsetOf(setB);
}

export function useSynchronizedAndMemoizedArticleData(article: ExpandedArticle) {
  const rawDataCache = useRef({
    script: null as string | null,
    cleanupScript: null as string | null,
    content: "",
  });

  const cache = useRef({
    version: 0,
    tags: [] as ArticleTag[],
    content: "",
    script: null as string | null,
    cleanupScript: null as string | null,
  });

  let hasChanged = false;

  if (rawDataCache.current.content !== article.content) {
    rawDataCache.current.content = article.content;

    const newArticleContent = decodeQuotes(article.content);
    if (!contentsAreEquivalent(cache.current.content, newArticleContent)) {
      cache.current.content = newArticleContent;
      hasChanged = true;
    }
  }

  if (!arraysAreEquivalent(cache.current.tags, article.tags)) {
    cache.current.tags = article.tags;
    hasChanged = true;
  }

  if (rawDataCache.current.script !== (article.script ?? null)) {
    rawDataCache.current.script = article.script ?? null;

    let script: string | null = article.script?.trim() ?? "";
    script = script === "" ? null : decodeQuotes(script);

    if (!nullableContentsAreEquivalent(cache.current.script, script)) {
      cache.current.script = script;
      hasChanged = true;
    }
  }

  if (rawDataCache.current.cleanupScript !== (article.cleanupScript ?? null)) {
    rawDataCache.current.cleanupScript = article.cleanupScript ?? null;

    let cleanupScript: string | null = null;
    if (cache.current.script) {
      const _cleanupScript = article.cleanupScript?.trim() ?? "";
      cleanupScript = _cleanupScript === "" ? null : decodeQuotes(_cleanupScript);
    }

    if (!nullableContentsAreEquivalent(cache.current.cleanupScript, cleanupScript)) {
      cache.current.cleanupScript = cleanupScript;
      hasChanged = true;
    }
  }

  if (hasChanged) {
    cache.current = { ...cache.current };
    cache.current.version++;
  }

  return cache.current;
}
