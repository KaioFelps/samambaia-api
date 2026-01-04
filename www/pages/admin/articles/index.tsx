import type { PageProps } from "@inertiajs/core/types";
import { router } from "@inertiajs/react";
import { useState } from "react";

import { Alert } from "@/components/alert";
import Button from "@/components/button";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import Pagination from "@/components/pagination";
import { routes } from "@/config/routes";
import { useMemoizedPaginatorParameters } from "@/hooks/pagination";
import { useCanSee } from "@/hooks/useCanSee";
import type { ArticlePreview } from "@/types/article-preview";
import { Permission } from "@/types/auth";
import type { Paginated } from "@/types/pagination";
import { ArticleListCard } from "@/ui/admin/article-list-card";
import TableHeader from "@/ui/admin/paginated-table-header";

type AdminArticleHomeProps = PageProps & {
  articles: Paginated<ArticlePreview[]>;
};

export default function AdminArticleHome({ articles, flash }: AdminArticleHomeProps) {
  const paginatorArgs = useMemoizedPaginatorParameters({
    lastPage: articles.pagination.totalPages,
    visibleButtons: 7,
    currentPage: articles.pagination.currentPage,
    align: "left",
  });

  const [filterIsLoading, setFilterIsLoading] = useState(false);
  const userCanCreateArticle = useCanSee(Permission.CreateArticle);

  function handleFilter({ filter, query }: { filter?: string; query?: string }) {
    const destination = !filter || !query ? "?" : `?${filter}=${query}`;

    router.visit(destination, {
      only: ["articles"],
      onStart: () => setFilterIsLoading(true),
      onFinish: () => setFilterIsLoading(false),
    });
  }

  return (
    <>
      <Head admin title="Notícias" />

      <Main admin>
        <Header.Root className="mb-8">
          <Header.Title>Notícias publicadas</Header.Title>
          <Header.Divisor />
          <Header.Actions>
            {userCanCreateArticle && (
              <Button admin asLink href={routes.admin.articles.create} variant="default">
                Criar notícia
              </Button>
            )}
          </Header.Actions>
        </Header.Root>

        {"createArticleSuccess" in flash && (
          <Alert admin message={flash.createArticleSuccess} type="success" className="mb-6" />
        )}

        {"deleteArticleSuccess" in flash && (
          <Alert admin message={flash.deleteArticleSuccess} type="success" className="mb-6" />
        )}

        <TableHeader.Root className="mb-4">
          <TableHeader.Count
            currentPage={articles.pagination.currentPage}
            itemsInCurrentPage={articles.data.length}
            itemsPerPage={articles.pagination.itemsPerPage}
            totalItems={articles.pagination.totalItems}
            subject="notícias"
          />
          <TableHeader.Filter
            filters={[
              { label: "Título", value: "title" },
              { label: "Autor", value: "author" },
            ]}
            handleFilter={handleFilter}
            loading={filterIsLoading}
          />
        </TableHeader.Root>

        {articles.pagination.totalItems > 0 ? (
          <div className="flex flex-col gap-1">
            {articles.data.map((article) => (
              <ArticleListCard key={`admin-article-list-${article.id}`} {...article} />
            ))}
          </div>
        ) : (
          <Alert admin type="info" message="Não há notícias publicadas." />
        )}

        {paginatorArgs && (
          <Pagination.Root paginatorArgs={paginatorArgs}>
            <Pagination.Container className="mt-8 justify-end">
              <Pagination.ArrowButton admin direction="backward" />
              <Pagination.Buttons admin />
              <Pagination.ArrowButton admin direction="forward" />
            </Pagination.Container>
          </Pagination.Root>
        )}
      </Main>
    </>
  );
}
