import type { PageProps } from "@inertiajs/core/types";
import { usePage } from "@inertiajs/react";
import PublicButton from "@/components/button/public-button/index";
import { Head } from "@/components/head";
import { Main } from "@/components/main";
import { Sprite } from "@/components/sprite";
import type { ExpandedArticle } from "@/types/expanded-article";
import { FaceGesture, Imager } from "@/utils/imager";

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
        <article id="article-container" className="card p-0 mb-2">
          <div id="article-header-wrapper" className="p-3">
            <header id="article-header-container" className="section-header blue mb-1">
              <h1>{props.article.title}</h1>
            </header>

            {props.article.tags.length > 0 && (
              <div id="article-tags-container" className="flex flex-wrap items-row gap-1 mb-3">
                {props.article.tags.map((tag) => (
                  <span key={`article-tag-${tag.id}`}>{tag.value}</span>
                ))}
              </div>
            )}
          </div>

          <hr id="article-divisor" className="mx-0.5" />

          <div
            id="article-content-container"
            className="p-3 article-content"
            // biome-ignore lint/security/noDangerouslySetInnerHtml: this content is curated by fansite staff
            dangerouslySetInnerHTML={{ __html: props.article.content }}
          />
        </article>

        <div id="article-footer-container" className="card py-0 flex items-center justify-between">
          <div id="article-author-container" className="flex gap-1 items-center">
            <div
              style={{
                background: `url(${Imager.getUserImage(props.article.author.nickname, {
                  gesture: FaceGesture.smile,
                  direction: "2",
                  head_direction: "3",
                  size: "m",
                  img_format: "png",
                })}) no-repeat calc((90px - 64px) / 2 * -1) -14px`,
              }}
              className="pixelated w-16 h-[72px]"
            />
            <div className="flex flex-col justify-center items-start text-sm text-gray-700">
              <span className="flex gap-0.5 items-center">
                <Sprite width={16} height={17} x={-139} y={-64} />
                Escrito por <strong>{props.article.author.nickname}</strong>
              </span>
              <span className="flex gap-0.5 items-center">
                <Sprite width={16} height={17} x={-128} y={-95} />
                Publicado em{" "}
                <strong>{new Date(props.article.createdAt).toLocaleDateString("pt-BR")}</strong>
              </span>
            </div>
          </div>

          <div id="article-actions-container" className="py-3 flex items-center justify-end gap-2">
            <PublicButton.Default variant="success" size="lg">
              <Sprite width={18} height={13} x={-94} y={-96} />
              Formulário
            </PublicButton.Default>

            <PublicButton.Default variant="yellow" size="lg">
              <Sprite width={16} height={14} x={-112} y={-64} /> 1
            </PublicButton.Default>
          </div>
        </div>

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
