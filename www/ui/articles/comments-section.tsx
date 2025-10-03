import { PublicAlert } from "@/components/alert/public-alert";
import type { Auth } from "@/types/auth";
import type { Comment } from "@/types/comment";
import type { Pagination } from "@/types/pagination";
import { CommentBox } from "./comment-box";
import { CreateCommentForm } from "./comment-form";
import { CommentsPagination } from "./comments-pagination";

type Props = {
  pagination: Pagination;
  comments: Comment[];
  articleId: string;
  auth?: Auth;
};

export function CommentsSection({ auth, comments, articleId, pagination }: Props) {
  return (
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
        <>
          <div className="flex flex-col gap-2">
            {comments.map((comment) => (
              <CommentBox key={`article-comment-${comment.id}`} auth={auth} {...comment} />
            ))}
          </div>

          {pagination.totalPages > 1 && <CommentsPagination {...pagination} />}
        </>
      ) : (
        <PublicAlert
          message="Ainda não há comentários. Seja o primeiro a dizer algo legal!"
          type="info"
        />
      )}
    </section>
  );
}
