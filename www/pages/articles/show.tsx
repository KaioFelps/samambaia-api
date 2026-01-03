import type { PageProps } from "@inertiajs/core/types";
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
import { useSynchronizedAndMemoizedArticleData } from "./hooks";

export type ShowArticleProps = PageProps & {
  article: ExpandedArticle;
  feed: Paginated<ArticlePreview[]>;
};

export default function ShowArticle(props: ShowArticleProps) {
  const {
    content,
    cleanupScript,
    script,
    tags,
    version: articleMemoizationVersion,
  } = useSynchronizedAndMemoizedArticleData(props.article);

  return (
    <>
      <Head
        title={props.article.title}
        description={props.article.description}
        cover={props.article.coverUrl}
      />

      <Main className="flex-1 max-w-main-center-content basis-main-center-content">
        <ArticleContainer
          key={`article-content-memoization-v${articleMemoizationVersion}-content`}
          title={props.article.title}
          tags={tags}
          content={content}
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

      <ArticleScript
        key={`article-content-memoization-v${articleMemoizationVersion}-script`}
        script={script}
        cleanupScript={cleanupScript}
      />
    </>
  );
}
