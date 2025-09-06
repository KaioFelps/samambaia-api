import { router } from "@inertiajs/react";
import { Trash } from "@phosphor-icons/react/dist/ssr/Trash";
import { memo, useCallback, useState } from "react";
import { toast } from "react-toastify";

import Dialog from "@/components/admin/dialog";
import Button from "@/components/button";
import { IconButton } from "@/components/icon-button";
import { useCanSee } from "@/hooks/useCanSee";
import { Permission } from "@/types/auth";

type DeleteArticleButtonProps = {
  articleTitle: string;
  articleId: string;
};

export const DeleteArticleButton = memo(({ articleId, articleTitle }: DeleteArticleButtonProps) => {
  const [processing, setProcessing] = useState(false);

  const handleDeleteArticleAndComments = useCallback(() => {
    const endpoint = `/gremio/noticias/${articleId}/apagar`;
    router.delete(endpoint, {
      onStart: () => setProcessing(true),
      onFinish: () => setProcessing(false),
      onError: (errors) => {
        if (!("error" in errors)) return;

        console.error(errors.error);
        toast(`Não foi possível apagar a notícia "${articleTitle}". Contate um desenvolvedor.`, {
          type: "error",
        });
      },
      onSuccess: () => toast(`Notícia ${articleTitle} apagada.`, { type: "success" }),
    });
  }, [articleId, articleTitle]);

  if (!useCanSee(Permission.DeleteArticle)) return null;

  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <IconButton admin size="sm" theme="danger" variant="ghost" icon={Trash} />
      </Dialog.Trigger>

      <Dialog.Content>
        <Dialog.Header
          title={`Apagar notícia "${articleTitle}"`}
          description={`Formulário para apagar a notícia ${articleTitle}`}
        />

        <Dialog.Container className="flex flex-col gap-4 text-gray-800">
          <p className="font-light">
            Ao confirmar sua intenção de apagar esta notícia, você se responsabiliza por removê-la{" "}
            <strong>permanentemente</strong> do banco de dados do fã-site, juntamente de todos os
            comentários associados à essa notícia.
          </p>

          <p className="font-light">Você tem certeza do que está fazendo?</p>

          <hr />

          <div className="flex items-center gap-2">
            <Button
              admin
              variant="default"
              theme="danger"
              size="lg"
              disabled={processing}
              onClick={handleDeleteArticleAndComments}>
              {processing ? "Apagando..." : "Estou ciente e me responsabilizo"}
            </Button>
            <Dialog.Close asChild>
              <Button admin size="lg" disabled={processing}>
                Deixa baixo
              </Button>
            </Dialog.Close>
          </div>
        </Dialog.Container>
      </Dialog.Content>
    </Dialog.Root>
  );
});
