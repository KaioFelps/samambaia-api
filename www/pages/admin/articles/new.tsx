import { useForm } from "@inertiajs/react";
import { ClipboardIcon } from "@phosphor-icons/react/dist/ssr/Clipboard";
import { PlusIcon } from "@phosphor-icons/react/dist/ssr/Plus";
import { SpinnerIcon } from "@phosphor-icons/react/dist/ssr/Spinner";
import type { FormEvent } from "react";
import { toast } from "react-toastify";
import tinymce from "tinymce";

import MultiSelect, { type SelectOption } from "@/components/admin/multiselect";
import Button from "@/components/button";
import Form from "@/components/form";
import { ValidationErrorSpan } from "@/components/form/validation-error-alert";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import { useCanSee } from "@/hooks/useCanSee";
import type { ArticleTag } from "@/types/article-tag";
import { Permission } from "@/types/auth";
import { TinyMCEEditor } from "@/ui/admin/tiny-mce-editor";
import { copyHtmlToClipboard } from "./shared";

type AdminCreateArticlePageProps = {
  tags: ArticleTag[];
};

type CreateArticleForm = {
  title: string;
  content: string;
  cover_url: string;
  description: string;
  tags: number[];
  author_id?: string;
};

export default function AdminCreateArticlePage({ tags }: AdminCreateArticlePageProps) {
  const { data, setData, errors, clearErrors, processing, post } = useForm<CreateArticleForm>({
    content: "",
    cover_url: "",
    description: "",
    title: "",
    tags: [],
  });

  const tagsOptions = tags.map(
    (tag) => ({ label: tag.value, value: tag.id.toString() }) satisfies SelectOption,
  );

  const userCanPublishInNameOfOthers = useCanSee(Permission.ChangeArticleAuthor);

  const handleCopyHtml = async () => {
    await copyHtmlToClipboard(tinymce);
  };

  const handleCreateArticle = (e: FormEvent) => {
    e.preventDefault();
    clearErrors();

    post("/gremio/noticias/criar", {
      onSuccess: () => {
        const timer = 3000;

        toast("Notícia criada com sucesso! Peça para que a revisem.", {
          type: "success",
          autoClose: timer,
        });
      },
      onError: (errors) => {
        if ("error" in errors) {
          console.error(errors.error);
          toast("Não foi possível publicar a notícia. Por favor, contate um desenvolvedor.", {
            type: "error",
          });
        }
      },
    });
  };

  return (
    <>
      <Head admin title="Nova notícia" />

      <Main admin>
        <Header.Root className="mb-8">
          <Header.Title>Criar notícia</Header.Title>
          <Header.Divisor />
        </Header.Root>

        <Form.Root onSubmit={handleCreateArticle} className="flex flex-col gap-3">
          <Form.Input
            admin
            label="Título"
            placeholder="Abracadabra"
            name="title"
            required
            validationError={errors.title}
            onInput={(e) => setData({ ...data, title: e.currentTarget.value })}
          />

          <Form.Input
            admin
            label="Descrição"
            name="description"
            placeholder="Amor, oh-na-na. Abra. Cadabra. Morta, oh-ga-ga."
            required
            validationError={errors.description}
            onInput={(e) => setData({ ...data, description: e.currentTarget.value })}
          />

          <Form.Input
            admin
            label="Topstory"
            placeholder="https://i.imgur.com/..."
            name="cover_url"
            required
            validationError={errors.cover_url}
            onInput={(e) => setData({ ...data, cover_url: e.currentTarget.value })}
          />

          {userCanPublishInNameOfOthers && (
            <div>
              <Form.Input
                admin
                label="ID do autor"
                placeholder="37824kef-vduih27i2-4289328v-489uf"
                name="author_id"
                validationError={errors.author_id}
                onInput={(e) => {
                  const value = e.currentTarget.value;
                  const authorId = value || undefined;
                  setData({ ...data, author_id: authorId });
                }}
              />
              <p className="text-sm font-light ml-1 text-gray-800">
                Ao preencher esse campo, a notícia será publicada no usuário com o ID especificado.
              </p>
            </div>
          )}

          <div>
            <label htmlFor="create-article-tags-select" className="block text-sm mb-1 ml-1">
              Tag/Categoria
            </label>
            <ValidationErrorSpan validationError={errors.tags} />
            <MultiSelect
              id="create-article-tags-select"
              options={tagsOptions}
              setValues={(value) =>
                setData({ ...data, tags: value.map((tag) => Number(tag.value)) })
              }
            />
          </div>

          <TinyMCEEditor
            validationError={errors.content}
            onEditorChange={(content) => setData({ ...data, content })}
          />

          <div className="mt-3 flex items-center gap-1.5">
            <Button admin variant="default" theme="success" size="lg" disabled={processing}>
              {processing ? (
                <>
                  <SpinnerIcon size={16} weight="bold" className="animate-spin" />
                  Publicando...
                </>
              ) : (
                <>
                  <PlusIcon size={16} weight="bold" />
                  Publicar
                </>
              )}
            </Button>
            <Button admin type="button" size="lg" onClick={handleCopyHtml}>
              <ClipboardIcon size={16} weight="bold" />
              Copiar HTML
            </Button>
          </div>
        </Form.Root>
      </Main>
    </>
  );
}
