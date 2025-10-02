import type { FormDataErrors } from "@inertiajs/core/types";
import { useForm } from "@inertiajs/react";
import { type FormEvent, useMemo } from "react";
import { toast } from "react-toastify";
import { Alert } from "@/components/alert";
import PublicButton from "@/components/button/public-button";
import { routes } from "@/config/routes";
import { Imager } from "@/utils/imager";

type Props = {
  isAuthenticated: boolean;
  userNickname: string | undefined;
  articleId: string;
};

type FormData = {
  content: string;
  error?: never;
};

export function CreateCommentForm({ isAuthenticated, userNickname, articleId }: Props) {
  const userHead = useMemo(() => {
    if (!isAuthenticated) return;

    return Imager.getUserImage(userNickname!, {
      headonly: "1",
      head_direction: "3",
      size: "m",
    });
  }, [userNickname, isAuthenticated]);

  const { setData, post, errors, clearErrors, resetAndClearErrors, data, processing } =
    useForm<FormData>({
      content: "",
    });

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    clearErrors();

    post(routes.web.comment.createComment(articleId), {
      preserveScroll: true,
      onSuccess: (page) => {
        resetAndClearErrors();
        const successFlashMessage: string | undefined = page.props.flash.commentSuccess;
        if (successFlashMessage) toast(successFlashMessage, { type: "success" });
      },
      onError: (errors: FormDataErrors<FormData>) => {
        if (errors.error) toast("Não foi possível registrar seu comentário.", { type: "error" });
        if (errors.content) toast(errors.content, { type: "error" });
      },
    });
  };

  if (!isAuthenticated) {
    return <Alert message="Você precisa estar logado para poder comentar." type="warning" />;
  }

  return (
    <form onSubmit={handleSubmit} className="flex items-center gap-3">
      <label className="w-full flex items-center gap-3">
        <img
          src={userHead}
          alt="Avatar do usuário"
          className="pixelated w-14 h-16 object-none object-[-24px_-20px]"
        />

        <input
          type="text"
          className="text-input w-full"
          placeholder="Comente algo legal..."
          required
          min={1}
          onInput={(event) => setData({ content: event.currentTarget.value })}
          value={data.content}
          data-invalid={errors.content}
        />
      </label>

      <PublicButton.Default disabled={processing} aria-busy={processing} type="submit" size="lg">
        Enviar
      </PublicButton.Default>
    </form>
  );
}
