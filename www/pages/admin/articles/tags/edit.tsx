import type { PageProps } from "@inertiajs/core/types";
import { router, useForm } from "@inertiajs/react";
import { PlusIcon } from "@phosphor-icons/react/dist/ssr/Plus";
import { SpinnerIcon } from "@phosphor-icons/react/dist/ssr/Spinner";
import type { FormEvent, PropsWithChildren } from "react";
import { toast } from "react-toastify";
import { AdminAlert } from "@/components/alert/admin-alert";
import { AdminButton } from "@/components/button/admin-button";
import Form from "@/components/form/admin-form";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import { routes } from "@/config/routes";
import type { ArticleTag } from "@/types/article-tag";

type Props = PageProps & {
  tag: ArticleTag | null;
};

type CreateTagForm = {
  value?: string;
};

export default function AdminEditArticleTagPage({ tag }: Props) {
  const { data, setData, errors, clearErrors, processing, put, transform } = useForm<CreateTagForm>(
    { value: tag?.value },
  );

  if (!tag) {
    return (
      <Wrapper title="Tag não encontrada">
        <AdminAlert type="warning" message="A tag especificada não foi encontrada." />
      </Wrapper>
    );
  }

  transform((data) => {
    if (data.value === tag.value) delete data.value;
    return data;
  });

  const handleCreateTag = (e: FormEvent) => {
    e.preventDefault();
    clearErrors();
    put(routes.admin.tags.updateChanges(tag.id), {
      onSuccess: (page) => {
        toast.success(page.props.flash.updateArticleTagSuccess, { autoClose: 3000 });
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
    <Wrapper title={`Editando tag ${tag.value}`}>
      <Form.Root onSubmit={handleCreateTag} className="flex flex-col gap-3">
        <Form.Input
          label="Valor"
          placeholder="Fã-site"
          name="value"
          required
          validationError={errors.value}
          onInput={(e) => setData({ value: e.currentTarget.value })}
          defaultValue={data.value}
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
                Salvando...
              </>
            ) : (
              <>
                <PlusIcon size={16} weight="bold" />
                Salvar
              </>
            )}
          </AdminButton>
          <AdminButton
            onClick={() => window.history.back()}
            size="lg"
            variant="ghost"
            type="button">
            Cancelar
          </AdminButton>
        </div>
      </Form.Root>
    </Wrapper>
  );
}

function Wrapper({ children, title }: PropsWithChildren<{ title: string }>) {
  return (
    <>
      <Head admin title={title} />

      <Main admin>
        <Header.Root className="mb-8">
          <Header.Title>{title}</Header.Title>
          <Header.Divisor />
        </Header.Root>

        {children}
      </Main>
    </>
  );
}
