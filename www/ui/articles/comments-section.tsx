import { router } from "@inertiajs/react";
import { useEffect, useEffectEvent, useState } from "react";
import { PublicAlert } from "@/components/alert/public-alert";
import Pagination from "@/components/pagination";
import type { Auth } from "@/types/auth";
import type { Comment } from "@/types/comment";
import type { Pagination as TPagination } from "@/types/pagination";
import type { Paginator } from "@/utils/paginator";
import { PaginatorBuilder } from "@/utils/paginator/builder";
import { CommentBox } from "./comment-box";
import { CreateCommentForm } from "./comment-form";

type Props = {
  pagination: TPagination;
  comments: Comment[];
  articleId: string;
  auth?: Auth;
};

export function CommentsSection({ auth, comments, articleId, pagination }: Props) {
  const [paginator, setPaginator] = useState<Paginator | null>(null);

  const updatePaginator = useEffectEvent(() => {
    setPaginator(
      new PaginatorBuilder()
        .ignoreOverflowErrors()
        .setVisibleButtons(7)
        .setAlign("left")
        .setLastPage(pagination.totalPages)
        .setCurrentPage(pagination.currentPage)
        .setPageQuery("commentsPage")
        .build(),
    );
  });

  const rewindPaginationIfOverflowed = useEffectEvent((pagination: TPagination) => {
    const commentsPage = Number(new URL(window.location.href).searchParams.get("commentsPage"));
    if (commentsPage <= pagination.totalPages || !paginator) return;
    router.visit(paginator.getPaginationLinkForPage(pagination.totalPages).link, {
      preserveScroll: true,
    });
  });

  useEffect(() => {
    updatePaginator();
    rewindPaginationIfOverflowed(pagination);
  }, [pagination]);

  if (!paginator) return null;

  return (
    <Pagination.Root paginator={paginator}>
      <section className="card flex flex-col gap-3">
        <header className="section-header gray">
          <h2>Comentários</h2>
        </header>

        <CreateCommentForm
          isAuthenticated={!!auth}
          userNickname={auth?.user.nickname}
          articleId={articleId}
        />

        <hr />

        {comments.length >= 1 ? (
          <div className="flex flex-col gap-2">
            {comments.map((comment) => (
              <CommentBox key={`article-comment-${comment.id}`} auth={auth} {...comment} />
            ))}
          </div>
        ) : (
          <PublicAlert
            message="Ainda não há comentários. Seja o primeiro a dizer algo legal!"
            type="info"
          />
        )}

        {pagination.totalPages > 1 && (
          <div>
            <Pagination.Container className="mt-3 justify-end">
              <Pagination.ArrowButton preserveScroll direction="backward" />
              <Pagination.Buttons preserveScroll />
              <Pagination.ArrowButton preserveScroll direction="forward" />
            </Pagination.Container>
          </div>
        )}
      </section>
    </Pagination.Root>
  );
}

export default CommentsSection;
