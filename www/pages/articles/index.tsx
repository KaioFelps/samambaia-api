import type { PageProps } from "@inertiajs/core/types";
import { usePage } from "@inertiajs/react";
import { Head } from "@/components/head";
import { Main } from "@/components/main";
import type { ExpandedArticle } from "@/types/expanded-article";
import { ArticleContainer } from "@/ui/articles/article-container";
import { ArticleFooter } from "@/ui/articles/article-footer";
import { Comments } from "@/ui/articles/comments";

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
        <ArticleContainer {...props.article} />

        <ArticleFooter
          authorNickname={props.article.author.nickname}
          publishmentDate={props.article.createdAt}
        />

        <Comments
          comments={props.article.comments.data}
          userNickname={props.auth?.user.nickname}
          isAuthenticated={Boolean(props.auth)}
          articleId={props.article.id}
        />
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
