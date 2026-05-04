import type { PageProps, SharedPageProps } from "@inertiajs/core/types";
import type { PropsWithChildren } from "react";
import { Alert } from "@/components/alert";
import Button from "@/components/button";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import { routes } from "@/config/routes";
import { useCanSee } from "@/hooks/useCanSee";
import type { ArticleTag } from "@/types/article-tag";
import { Permission } from "@/types/auth";
import type { Paginated } from "@/types/pagination";
import { ArticleTagListCard } from "@/ui/admin/article-tag-list-card";
import TableHeader from "@/ui/admin/paginated-table-header";

type AdminArticleTagsPageProps = SharedPageProps &
  PageProps & {
    tags: Paginated<ArticleTag[]>;
  };

export default function AdminArticleTagsHome({ tags }: AdminArticleTagsPageProps) {
  if (tags.pagination.totalItems === 0) {
    return (
      <Wrapper>
        <Alert admin type="info" message="Ainda não existem tags de notícias." />
      </Wrapper>
    );
  }

  return (
    <Wrapper>
      <TableHeader.Root className="mb-4">
        <TableHeader.Count
          currentPage={tags.pagination.currentPage}
          itemsInCurrentPage={tags.data.length}
          itemsPerPage={tags.pagination.itemsPerPage}
          totalItems={tags.pagination.totalItems}
          subject="tags"
        />
      </TableHeader.Root>
      <div className="flex flex-col gap-1">
        {tags.data.map((tag) => (
          <ArticleTagListCard key={`admin-article-tag-list-${tag.id}`} tag={tag} />
        ))}
      </div>
    </Wrapper>
  );
}

function Wrapper({ children }: PropsWithChildren) {
  const userCanCreateTags = useCanSee(Permission.CreateArticleTag);
  return (
    <>
      <Head admin title="Tags" />
      <Main admin>
        <Header.Root className="mb-8">
          <Header.Title>Tags de notícias</Header.Title>
          <Header.Divisor />
          <Header.Actions>
            {userCanCreateTags && (
              <Button admin asLink href={routes.admin.tags.create} variant="default">
                Criar nova tag
              </Button>
            )}
          </Header.Actions>
        </Header.Root>

        {children}
      </Main>
    </>
  );
}
