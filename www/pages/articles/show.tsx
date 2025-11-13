import type { PageProps } from "@inertiajs/core/types";
import { useMemo } from "react";
import { Head } from "@/components/head";
import { Main } from "@/components/main";
import type { ArticlePreview } from "@/types/article-preview";
import type { ExpandedArticle } from "@/types/expanded-article";
import type { Paginated } from "@/types/pagination";
import { ArticleContainer } from "@/ui/articles/article-container";
import { ArticleFeed } from "@/ui/articles/article-feed";
import { ArticleFooter } from "@/ui/articles/article-footer";
import { ArticleScript } from "@/ui/articles/article-script";
import { CommentsSection } from "@/ui/articles/comments-section";
import { decodeQuotes } from "@/utils/quotes";

type Props = PageProps & {
  article: ExpandedArticle;
  feed: Paginated<ArticlePreview[]>;
};

export default function ShowArticle(props: Props) {
  const articleContent = useMemo(
    () => decodeQuotes(props.article.content),
    [props.article.content],
  );

  const script = useMemo(() => {
    const script = props.article.script?.trim() ?? "";
    return script === "" ? null : decodeQuotes(script);
  }, [props.article.script]);

  const cleanupScript = useMemo(() => {
    if (!script) return null;
    const cleanupScript = props.article.cleanupScript?.trim() ?? "";
    return cleanupScript === "" ? null : decodeQuotes(cleanupScript);
  }, [props.article.cleanupScript, script]);

  return (
    <>
      <Head
        title={props.article.title}
        description={props.article.description}
        cover={props.article.coverUrl}
      />

      <Main className="flex-1 max-w-main-center-content basis-main-center-content">
        <ArticleContainer
          title={props.article.title}
          tags={props.article.tags}
          content={articleContent}
        />

        <ArticleFooter
          authorNickname={props.article.author.nickname}
          publishmentDate={props.article.createdAt}
        />

        <CommentsSection
          pagination={props.article.comments.pagination}
          comments={props.article.comments.data}
          auth={props.auth}
          articleId={props.article.id}
        />
      </Main>

      <ArticleFeed {...props.feed} />

      {script && <ArticleScript script={script} cleanupScript={cleanupScript} />}
    </>
  );
}
