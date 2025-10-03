import type { Auth } from "@/types/auth";
import type { Comment } from "@/types/comment";
import { CommentBox } from "./comment-box";
import { CreateCommentForm } from "./comment-form";

type Props = {
  comments: Comment[];
  articleId: string;
  auth?: Auth;
};

export function CommentsSection({ auth, comments, articleId }: Props) {
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
      <div className="flex flex-col gap-2">
        {comments.map((comment) => (
          <CommentBox key={`article-comment-${comment.id}`} auth={auth} {...comment} />
        ))}
      </div>
    </section>
  );
}
