import type { PageProps } from "@inertiajs/core/types";
import { Link } from "@inertiajs/react";
import { useEffect, useEffectEvent } from "react";
import { toast } from "react-toastify";
import { Alert } from "@/components/alert";
import { ArticleCard } from "@/components/article-card";
import PublicButton from "@/components/button/public-button";
import { Head } from "@/components/head";
import { Main } from "@/components/main";
import { Sprite } from "@/components/sprite";
import { useArchiveFilters } from "@/pages/articles/hooks/use-archive-filters";
import type { ArticlePreview } from "@/types/article-preview";
import type { ArticleTag } from "@/types/article-tag";
import type { Paginated } from "@/types/pagination";
import { ArticleArchiveFilters } from "@/ui/articles/article-archive-filters";

export type ArticlesArchiveProps = PageProps & {
  feed: Paginated<ArticlePreview[]>;
  tags: ArticleTag[];
  queryErrors: Record<string, string>;
};

const queryErrorsFieldsToName = {
  includingTags: "Tag selecionada",
  including_tags: "Tag selecionada",
  page: "Página",
  title: "Título",
  author: "Autor",
};

export default function ArticlesArchive({ feed, tags, queryErrors }: ArticlesArchiveProps) {
  const archiveFilters = useArchiveFilters();
  const { processing, goToPage } = archiveFilters;
  const { currentPage, totalPages, totalItems } = feed.pagination;

  const toastQueryErrors = useEffectEvent(() => {
    Object.entries(queryErrors).forEach(([key, message]) => {
      const field = queryErrorsFieldsToName[key as keyof typeof queryErrorsFieldsToName];
      toast.warning(`Erro ao aplicar o filtro '${field}'. (${message})`);
    });
  });

  useEffect(() => {
    toastQueryErrors();
  }, []);

  return (
    <>
      <Head title="Arquivo de notícias" description="A biblioteca do jornalismo fã-site." />

      <Main className="card flex flex-col gap-3">
        <div id="article-header-wrapper">
          <header id="article-header-container" className="section-header blue mb-2">
            <h1>Arquivo de notícias</h1>
          </header>

          <ArticleArchiveFilters tags={tags} archiveFilters={archiveFilters} />
        </div>

        <hr id="article-divisor" className="mx-0.5" />

        <div className="flex items-center justify-between">
          <span className="font-bold text-lg">
            Resultados ({feed.data.length} de {totalItems})
          </span>

          <div className="flex gap-1">
            <button
              type="button"
              aria-label="Página anterior"
              disabled={processing || currentPage <= 1}
              onClick={() => goToPage(currentPage - 1)}
              className="btn-arrow blue">
              <Sprite x={-184} y={-62} height={20} width={14} />
            </button>
            <button
              type="button"
              aria-label="Próxima página"
              disabled={processing || currentPage >= totalPages}
              onClick={() => goToPage(currentPage + 1)}
              className="btn-arrow blue">
              <Sprite x={-184} y={-62} height={20} width={14} className="rotate-180" />
            </button>
          </div>
        </div>

        {feed.data.length > 0 ? (
          <div className="grid grid-flow-row grid-cols-2 gap-2">
            {feed.data.map((article) => (
              <ArticleCard key={`article-archive-card-${article.id}`} {...article} />
            ))}
          </div>
        ) : (
          <Alert className="mt-3" type="warning" message="Ups! Parece que ainda não há notícias." />
        )}
      </Main>

      <ArticleArchiveWarningsSidebar />
    </>
  );
}

function ArticleArchiveWarningsSidebar() {
  return (
    <aside className="card flex-1 h-fit sticky top-2">
      <section className="mb-3">
        <header className="section-header gray mb-3">
          <h2>Notícias</h2>
        </header>
        <p className="text-sm mb-3">
          As notícias publicadas no Habblive Lite são de qualidade e veridicidade, quando não
          artigos de opinião (artigos de opinião representam a opinião do jornalista, jamais do
          fã-site).
        </p>
        <p className="text-sm">
          Todos os artigos são revisados pelo editor-chefe e fontes são requeridas para garantir
          confiança aos leitores.
        </p>
      </section>
      <section>
        <header className="section-header gray mb-3">
          <h2>Denúncias</h2>
        </header>
        <p className="text-sm mb-3">
          Se houver a necessidade de reportar alguma notícia, quer por divulgar inverdades, quer por
          ofender usuários ou qualquer coisa que fira os valores do fã-site, basta preencher o
          formulário de reclamação.
        </p>
        <PublicButton.Default asChild variant="yellow">
          <Link href="#" role="button">
            <Sprite width={17} height={16} x={-31} y={-96} />
            Denunciar
          </Link>
        </PublicButton.Default>
      </section>
    </aside>
  );
}
