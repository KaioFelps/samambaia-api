import { Alert } from "@/components/alert";
import Button from "@/components/button";
import Header from "@/components/header";
import Pagination from "@/components/pagination";
import { useMemoizedPaginatorParameters } from "@/hooks/pagination";
import { ArticlePreview } from "@/types/article-preview";
import { Paginated } from "@/types/pagination";
import { ArticleListCard } from "@/ui/admin/article-list-card";

type AdminArticleHomeProps = {
  articles: Paginated<ArticlePreview[]>;
};

export default function AdminArticleHome({ articles }: AdminArticleHomeProps) {
  return (
    <main className="admin-main-container">
      <Header.Root className="mb-8">
        <Header.Title>Notícias publicadas</Header.Title>
        <Header.Divisor />
        <Header.Actions>
          <Button
            admin
            variant="default"
          >
            Criar notícia
          </Button>
        </Header.Actions>
      </Header.Root>

      {articles.pagination.totalItems > 0
        ? (
          <div className="flex flex-col gap-1">
            {articles.data.map((article) => (
              <ArticleListCard
                key={`admin-article-list-${article.id}`}
                {...article}
              />),
            )}
          </div>
          )
        : (
          <Alert
            admin
            type="info"
            message="Não há notícias publicadas."
          />
          )}

      <Pagination.Root
        paginator={useMemoizedPaginatorParameters({
          lastPage: articles.pagination.totalPages,
          visibleButtons: 7,
          currentPage: articles.pagination.currentPage,
          align: "left",
        })!}
        className="mt-8 justify-end"
      >
        <Pagination.ArrowButton
          admin
          direction="backward"
        />
        <Pagination.Buttons admin />
        <Pagination.ArrowButton
          admin
          direction="forward"
        />
      </Pagination.Root>
    </main>
  );
};
