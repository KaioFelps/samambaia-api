import type { Comment } from "@/types/comment";
import { CommentBox } from "./comment-box";
import { CreateCommentForm } from "./comment-form";

type Props = {
  comments: Comment[];
  articleId: string;
  userNickname: string | undefined;
  isAuthenticated: boolean;
};

export function Comments({ userNickname, isAuthenticated, comments, articleId }: Props) {
  return (
    <section className="card flex flex-col gap-3">
      <header className="section-header gray">
        <h2>Comentários</h2>
      </header>

      <CreateCommentForm
        isAuthenticated={isAuthenticated}
        userNickname={userNickname}
        articleId={articleId}
      />

      <hr />
      <div className="flex flex-col gap-2">{comments.map(CommentBox)}</div>
    </section>
  );
}
