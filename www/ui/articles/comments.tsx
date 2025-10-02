import { useMemo } from "react";
import { Alert } from "@/components/alert";
import PublicButton from "@/components/button/public-button";
import type { Comment } from "@/types/comment";
import { Imager } from "@/utils/imager";
import { CommentBox } from "./comment-box";

type Props = {
  userNickname: string | undefined;
  comments: Comment[];
};

export function Comments({ userNickname, comments }: Props) {
  const userHead = useMemo(() => {
    if (!userNickname) return null;
    return Imager.getUserImage(userNickname, {
      headonly: "1",
      head_direction: "3",
      size: "m",
    });
  }, [userNickname]);

  return (
    <section className="card flex flex-col gap-3">
      <header className="section-header gray">
        <h2>Comentários</h2>
      </header>

      {userNickname ? (
        <form className="flex items-center gap-3">
          <label className="w-full flex items-center gap-3">
            <img
              src={userHead!}
              alt="Avatar do usuário"
              className="pixelated w-14 h-16 object-none object-[-24px_-20px]"
            />

            <input
              type="text"
              className="text-input w-full"
              placeholder="Comente algo legal..."
              required
            />
          </label>

          <PublicButton.Default type="submit" size="lg">
            Enviar
          </PublicButton.Default>
        </form>
      ) : (
        <Alert message="Você precisa estar logado para poder comentar." type="warning" />
      )}

      <hr />
      <div className="flex flex-col gap-2">{comments.map(CommentBox)}</div>
    </section>
  );
}
