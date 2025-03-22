import { Alert } from "@/components/alert";
import Button from "@/components/button";
import Header from "@/components/header";
import { ArticlePreview } from "@/types/article-preview";
import { Pagination } from "@/types/pagination";
import { ArticleListCard } from "@/ui/admin/article-list-card";

type AdminArticleHomeProps = {
  articles: {
    data: ArticlePreview[];
    pagination: Pagination;
  };
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
    </main>
  );
}
