import { Link } from "@inertiajs/react";
import { memo } from "react";
import { PublicAlert } from "@/components/alert/public-alert";
import PublicButton from "@/components/button/public-button";
import { Sprite } from "@/components/sprite";
import type { Article } from "@/types/article";
import type { Paginated } from "@/types/pagination";
import { ArticleFeedLink } from "./article-feed-link";

type Props = Paginated<Article[]>;

export const ArticleFeed = memo(({ data, pagination }: Props) => {
  return (
    <aside className="card flex-1 h-fit sticky top-2">
      <header className="section-header gray mb-3">
        <h2>Feed de notícias</h2>
      </header>

      {data.length ? (
        <>
          <ul className="flex flex-col gap-0.5 mb-3">
            {data.map((article) => (
              <li key={`articles-feed-article-link-to-${article.id}`} className="list-none">
                <ArticleFeedLink title={article.title} slug={article.slug} />
              </li>
            ))}
          </ul>

          <PublicButton.Ghost asChild variant="blue">
            <Link href="#">
              <Sprite width={14} height={14} x={-193} y={-97} />
              Mais notícias
            </Link>
          </PublicButton.Ghost>
        </>
      ) : (
        <PublicAlert message="Ainda não há notícias publicadas." type="info" />
      )}
    </aside>
  );
});
