import type { PageProps } from "@inertiajs/core/types";
import { usePage } from "@inertiajs/react";
import { Head } from "@/components/head";
import { Main } from "@/components/main";
import type { ExpandedArticle } from "@/types/expanded-article";

type Props = PageProps & {
  article: ExpandedArticle;
};

export default function ShowArticle() {
  const props = usePage<Props>().props;

  return (
    <>
      <Head
        title={props.article.title}
        description={props.article.description}
        cover={props.article.coverUrl}
      />

      <Main className="flex-1 max-w-main-center-content basis-main-center-content">
        <article className="card p-0">
          <div className="p-3">
            <header className="section-header blue mb-1">
              <h1>{props.article.title}</h1>
            </header>

            {props.article.tags.length > 0 && (
              <div className="flex flex-wrap items-row gap-1 mb-3">
                {props.article.tags.map((tag) => (
                  <span key={`article-tag-${tag.id}`}>{tag.value}</span>
                ))}
              </div>
            )}
          </div>

          <hr className="mx-0.5" />

          <div
            className="p-3 article-content"
            // biome-ignore lint/security/noDangerouslySetInnerHtml: this content is curated by fansite staff
            dangerouslySetInnerHTML={{ __html: props.article.content }}
          />
        </article>

        {/* TODO: Comments */}
      </Main>

      <aside className="card flex-1 h-fit">
        <header className="section-header gray">
          <h2>Feed de notícias</h2>
        </header>
        {/* TODO: Articles Feed */}
      </aside>
    </>
  );
}
