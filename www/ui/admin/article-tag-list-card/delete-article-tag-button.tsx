import { router } from "@inertiajs/react";
import { TrashIcon } from "@phosphor-icons/react/dist/ssr/Trash";
import { useState } from "react";
import { toast } from "react-toastify";
import Dialog from "@/components/admin/dialog";
import { AdminButton } from "@/components/button/admin-button";
import { AdminIconButton } from "@/components/icon-button/admin-icon-button";
import { routes } from "@/config/routes";
import type { ArticleTag } from "@/types/article-tag";

type Props = {
  tagId: ArticleTag["id"];
  tagValue: string;
};

export function DeleteArticleTagButton({ tagId, tagValue }: Props) {
  const [isProcessing, setIsProcessing] = useState(false);

  const handleDeleteArticleTag = () => {
    router.delete(routes.admin.tags.delete(tagId), {
      preserveScroll: true,
      onSuccess: () => toast.success(`Tag ${tagValue} removida com sucesso.`),
      onError: (errors) => {
        toast.error(`Não foi possível remover a tag ${tagValue}.`);
        if ("error" in errors) console.error(errors.error);
      },
      onStart: () => setIsProcessing(true),
      onFinish: () => setIsProcessing(false),
    });
  };

  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <AdminIconButton size="sm" theme="danger" variant="ghost" icon={TrashIcon} />
      </Dialog.Trigger>

      <Dialog.Content>
        <Dialog.Header
          title={`Apagar tag "${tagValue}"`}
          description={`Formulário para apagar a notícia ${tagValue} de ID ${tagId}.`}
        />

        <Dialog.Container className="flex flex-col gap-4 text-gray-800">
          <p className="font-light">
            Ao apagar essa tag, você se responsabiliza pela sua remoção e desassociação com todas as
            notícias que estão relacionadas com ela. Essa ação é <strong>irreversível</strong>. Ao
          </p>

          <p className="font-light">Você tem certeza do que está fazendo?</p>

          <hr />

          <div className="flex items-center justify-end gap-2">
            <AdminButton
              variant="default"
              theme="danger"
              size="lg"
              disabled={isProcessing}
              onClick={handleDeleteArticleTag}>
              {isProcessing ? "Apagando..." : "Apagar"}
            </AdminButton>
            <Dialog.Close asChild>
              <AdminButton size="lg" disabled={isProcessing}>
                Deixa baixo
              </AdminButton>
            </Dialog.Close>
          </div>
        </Dialog.Container>
      </Dialog.Content>
    </Dialog.Root>
  );
}
