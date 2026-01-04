import { memo } from "react";
import { Chip } from "@/components/chip";
import type { ArticleTag } from "@/types/article-tag";

type Props = {
  title: string;
  content: string;
  tags: ArticleTag[];
};

export const ArticleContainer = memo(({ title, tags, content }: Props) => {
  return (
    <article id="article-container" className="card p-0 mb-2">
      <div id="article-header-wrapper" className="p-3">
        <header id="article-header-container" className="section-header blue mb-1">
          <h1>{title}</h1>
        </header>

        {tags.length > 0 && (
          <div id="article-tags-container" className="flex flex-wrap items-row gap-1">
            {tags.map((tag) => (
              <Chip key={`article-tag-${tag.id}`}>{tag.value}</Chip>
            ))}
          </div>
        )}
      </div>

      <hr id="article-divisor" className="mx-0.5" />

      <div
        id="article-content-container"
        className="p-3 article-content"
        // biome-ignore lint/security/noDangerouslySetInnerHtml: this content is curated by fansite staff
        dangerouslySetInnerHTML={{ __html: content }}
      />
    </article>
  );
});
