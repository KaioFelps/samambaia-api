import { colors } from "@crate/tailwind.config";
import { Head, Link, router } from "@inertiajs/react";
import { Plus } from "@phosphor-icons/react/dist/ssr/Plus";
import { memo, useCallback } from "react";

import { Alert } from "@/components/alert";
import { ArticleCard } from "@/components/articleCard";
import { BadgeCard } from "@/components/badgeCard";
import { Sprite } from "@/components/sprite";
import { type SharedProps } from "@/inertiaShared";
import { colorWithOpacity } from "@/lib/tailwind";
import { Article } from "@/types/article";
import { FreeBadge } from "@/types/freeBadge";
import { Pagination } from "@/types/pagination";

type HomeProps = SharedProps & {
  articles: Article[];
  freeBadges: { data: FreeBadge[]; pagination: Pagination };
};

export default function Index({ articles, freeBadges }: HomeProps) {
  return (
    <>
      <Head>
        <title>Início</title>
      </Head>

      <main className="flex-1 flex flex-col gap-2">
        <ArticlesSection articles={articles} />
        <FreeBadgeSection
          badges={freeBadges.data}
          currentPage={freeBadges.pagination.currentPage}
        />
      </main>
    </>

  );
}

const ArticlesSection = memo(({ articles }: { articles: Article[] }) => (
  <section className="card w-full">
    <header className="section-header blue flex gap-3 items-center justify-between mb-3">
      <h1>
        Últimas notícias
      </h1>

      <Link
        href="/"
        style={{
          boxShadow: `
            0 2px 0 0 ${colorWithOpacity(colors.white, 25)},
            inset 0 2px 0 0 ${colorWithOpacity(colors.white, 50)}`,
        }}
        className="
          flex items-center gap-1 text-white text-sm font-medium
          border-2 border-gray-800 bg-blue-500 px-2 py-[5.5px] rounded-[6px]
          leading-tight
          "
      >
        <Plus
          size={16}
          weight="bold"
        />
        Ver mais notícias
      </Link>
    </header>

    {articles.length > 0
      ? (
        <div className="grid grid-flow-row grid-cols-2 gap-2">
          {articles.map(ArticleCard)}
        </div>)
      : (
        <Alert
          className="mt-3"
          type="warning"
          message="Ups! Parece que ainda não há notícias."
        />
        )}
  </section>
));

type FreeBadgeSectionProps = { badges: FreeBadge[]; currentPage: number };

const FreeBadgeSection = memo(({ badges, currentPage }: FreeBadgeSectionProps) => {
  const PER_PAGE = 26;

  function goToPage(page: number) {
    router.visit(`/?fb_p=${page}`, { only: ["freeBadges"] });
  }

  const handleGoToPrevPage = useCallback(() => {
    goToPage(currentPage - 1 || 1);
  }, [currentPage]);

  const handleGoToNextPage = useCallback(() => {
    goToPage(currentPage + 1);
  }, [currentPage]);

  return (
    <section className="card w-full">
      <header className="section-header green flex gap-3 items-center justify-between mb-3">
        <h2>
          Emblemas grátis
        </h2>

        <div className="flex gap-1">
          <button
            className="btn-arrow-green"
            disabled={currentPage === 1}
            onClick={handleGoToPrevPage}
          >
            <Sprite
              x={-184}
              y={-62}
              height={20}
              width={14}
            />
          </button>
          <button
            className="btn-arrow-green"
            disabled={badges.length <= PER_PAGE}
            onClick={handleGoToNextPage}
          >
            <Sprite
              x={-184}
              y={-62}
              height={20}
              width={14}
              className="rotate-180"
            />
          </button>
        </div>
      </header>

      {badges.length > 0
        ? (

          <div className="grid grid-cols-[repeat(13,_minmax(0,_1fr))] grid-flow-row gap-1">
            {badges.map(BadgeCard)}
          </div>
          )
        : (
          <Alert
            className="mt-3"
            type="warning"
            message="Não tem nenhum emblema grátis!"
          />
          )}
    </section>
  );
});
