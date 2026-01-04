import { router, useForm } from "@inertiajs/react";
import { ArrowLeftIcon } from "@phosphor-icons/react/dist/ssr/ArrowLeft";
import { PlusIcon } from "@phosphor-icons/react/dist/ssr/Plus";
import { SpinnerIcon } from "@phosphor-icons/react/dist/ssr/Spinner";
import type { FormEvent } from "react";
import { toast } from "react-toastify";
import { AdminButton } from "@/components/button/admin-button";
import Form from "@/components/form/admin-form";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import { routes } from "@/config/routes";

type CreateTagForm = {
  value: string;
};

export default function AdminCreateArticleTagPage() {
  const { setData, errors, clearErrors, processing, post } = useForm<CreateTagForm>();

  const handleCreateTag = (e: FormEvent) => {
    e.preventDefault();
    clearErrors();
    post(routes.admin.tags.storeNewTag, {
      onSuccess: (page) => {
        toast.success(page.props.flash.createArticleTagSuccess, { autoClose: 3000 });
        router.visit(routes.admin.tags.list);
      },
      onError: (errors) => {
        if (!("error" in errors)) return;
        toast.error("Não foi possível criar a nova tag. Por favor, contate um desenvolvedor.");
        console.error(errors.error);
      },
    });
  };

  return (
    <>
      <Head admin title="Nova tag de notícia" />

      <Main admin>
        <Header.Root className="mb-8">
          <Header.Title>Criar tag de notícia</Header.Title>
          <Header.Divisor />
        </Header.Root>

        <Form.Root onSubmit={handleCreateTag} className="flex flex-col gap-3">
          <Form.Input
            label="Valor"
            placeholder="Fã-site"
            name="value"
            required
            validationError={errors.value}
            onInput={(e) => setData({ value: e.currentTarget.value })}
          />

          <div className="mt-3 flex items-center gap-1.5">
            <AdminButton
              variant="default"
              theme="success"
              size="lg"
              type="submit"
              disabled={processing}>
              {processing ? (
                <>
                  <SpinnerIcon size={16} weight="bold" className="animate-spin" />
                  Criando...
                </>
              ) : (
                <>
                  <PlusIcon size={16} weight="bold" />
                  Criar
                </>
              )}
            </AdminButton>
            <AdminButton onClick={() => window.history.back()} size="lg" type="button">
              <ArrowLeftIcon size={16} weight="bold" />
              Voltar
            </AdminButton>
          </div>
        </Form.Root>
      </Main>
    </>
  );
}
