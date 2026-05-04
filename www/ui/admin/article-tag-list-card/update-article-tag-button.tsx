import { useForm } from "@inertiajs/react";
import { PencilSimpleIcon } from "@phosphor-icons/react/dist/ssr/PencilSimple";
import { SpinnerIcon } from "@phosphor-icons/react/dist/ssr/Spinner";
import { type FormEvent, useEffect, useState } from "react";
import { toast } from "react-toastify";
import Dialog from "@/components/admin/dialog";
import { AdminButton } from "@/components/button/admin-button";
import Form from "@/components/form/admin-form";
import { AdminIconButton } from "@/components/icon-button/admin-icon-button";
import { routes } from "@/config/routes";
import type { ArticleTag } from "@/types/article-tag";

type Props = {
  tag: ArticleTag;
};

type CreateTagForm = {
  value?: string;
};

export function UpdateArticleTagButton({ tag }: Props) {
  const [isOpen, setIsOpen] = useState(false);
  const { data, setData, put, processing, errors, transform, reset, clearErrors } =
    useForm<CreateTagForm>({
      value: tag.value,
    });

  useEffect(() => {
    if (!isOpen) return;
    reset();
    clearErrors();
  }, [isOpen, reset, clearErrors]);

  transform((data) => {
    const newData = { ...data };
    if (newData.value === tag.value) delete newData.value;
    return newData;
  });

  const handleUpdateTag = (e: FormEvent) => {
    e.preventDefault();

    put(routes.admin.tags.updateChanges(tag.id), {
      preserveState: true,
      preserveScroll: true,
      onStart: () => clearErrors(),
      onSuccess: (page) => {
        toast.success(page.props.flash.updateArticleTagSuccess);
        setIsOpen(false);
      },
      onError: (errors) => {
        if (errors.error) toast.error(errors.error);
      },
    });
  };

  return (
    <Dialog.Root open={isOpen} onOpenChange={setIsOpen}>
      <Dialog.Trigger asChild>
        <AdminIconButton size="sm" variant="ghost" theme="warn" icon={PencilSimpleIcon} />
      </Dialog.Trigger>

      <Dialog.Content>
        <Dialog.Header
          title={`Modificar tag "${tag.value}"`}
          description={`Formulário para modificar a tag ${tag.value} (de ID ${tag.id}).`}
        />

        <Dialog.Container>
          <form onSubmit={handleUpdateTag} className="flex flex-col gap-4 text-gray-800">
            <Form.Input
              label="Valor"
              placeholder="Fã-site"
              name="value"
              required
              validationError={errors.value}
              onInput={(e) => setData({ value: e.currentTarget.value })}
              value={data.value}
            />

            <Dialog.ActionsFooter>
              <Dialog.Close asChild>
                <AdminButton size="lg" disabled={processing}>
                  Deixa baixo
                </AdminButton>
              </Dialog.Close>

              <AdminButton
                variant="default"
                theme="success"
                size="lg"
                type="submit"
                disabled={processing}>
                {processing ? (
                  <>
                    <SpinnerIcon size={16} weight="bold" className="animate-spin" />
                    Salvando...
                  </>
                ) : (
                  <>
                    <PencilSimpleIcon size={16} weight="bold" />
                    Salvar
                  </>
                )}
              </AdminButton>
            </Dialog.ActionsFooter>
          </form>
        </Dialog.Container>
      </Dialog.Content>
    </Dialog.Root>
  );
}
