import type { PageProps } from "@inertiajs/core/types";
import { useEffect, useId } from "react";
import { Head } from "@/components/head";
import { Main } from "@/components/main";
import type { Article } from "@/types/article";
import type { ExpandedArticle } from "@/types/expanded-article";
import type { Paginated } from "@/types/pagination";
import { ArticleContainer } from "@/ui/articles/article-container";
import { ArticleFeed } from "@/ui/articles/article-feed";
import { ArticleFooter } from "@/ui/articles/article-footer";
import { CommentsSection } from "@/ui/articles/comments-section";

type Props = PageProps & {
  article: ExpandedArticle;
  feed: Paginated<Article[]>;
};

function resolveScriptAsNull(rawScript?: string | null): string | null {
  const script = rawScript?.trim() ?? "";
  return script === "" ? null : script;
}

export default function ShowArticle(props: Props) {
  const scriptId = useId();
  useEffect(() => {
    const articleScript = resolveScriptAsNull(props.article.script);
    if (!articleScript) return;

    // this *is* a hack to get this inserted after the dom has been stabilized
    // it will be processed after dom has been finished being painted
    // see: https://stackoverflow.com/questions/779379/why-is-settimeoutfn-0-sometimes-useful
    const renderTimeout = setTimeout(() => {
      const scriptElement = document.createElement("script");
      scriptElement.textContent = articleScript;
      scriptElement.type = "module";
      scriptElement.id = scriptId;

      document.body.appendChild(scriptElement);
    }, 100);

    return () => {
      clearTimeout(renderTimeout);
      const scriptElement = document.getElementById(scriptId);
      if (scriptElement) document.body.removeChild(scriptElement);
    };
  }, [props.article.script, scriptId]);

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

        <CommentsSection
          pagination={props.article.comments.pagination}
          comments={props.article.comments.data}
          auth={props.auth}
          articleId={props.article.id}
        />
      </Main>

      <ArticleFeed {...props.feed} />
    </>
  );
}
