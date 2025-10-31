import { useForm } from "@inertiajs/react";
import { ClipboardIcon } from "@phosphor-icons/react/dist/ssr/Clipboard";
import { PlusIcon } from "@phosphor-icons/react/dist/ssr/Plus";
import { SpinnerIcon } from "@phosphor-icons/react/dist/ssr/Spinner";
import { type FormEvent, lazy, Suspense, useRef } from "react";
import { toast } from "react-toastify";
import type { Editor } from "tinymce";

import MultiSelect, { type SelectOption } from "@/components/admin/multiselect";
import Button from "@/components/button";
import Form from "@/components/form/admin-form";
import { ValidationErrorSpan } from "@/components/form/validation-error-alert";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import { useCanSee } from "@/hooks/useCanSee";
import type { ArticleTag } from "@/types/article-tag";
import { Permission } from "@/types/auth";
import { TinyMCEEditorSkeleton } from "@/ui/admin/tiny-mce-editor/skeleton";
import { copyHtmlToClipboard } from "./shared";

const TinyMCEEditor = lazy(() => import("@/ui/admin/tiny-mce-editor"));

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
  script?: string;
};

export default function AdminCreateArticlePage({ tags }: AdminCreateArticlePageProps) {
  const tinymce = useRef<Editor>(null);
  const { data, setData, errors, clearErrors, processing, post, transform } =
    useForm<CreateArticleForm>({
      content: "",
      cover_url: "",
      description: "",
      title: "",
      tags: [],
      script: "",
    });

  const tagsOptions = tags.map(
    (tag) => ({ label: tag.value, value: tag.id.toString() }) satisfies SelectOption,
  );

  const userCanPublishInNameOfOthers = useCanSee(Permission.ChangeArticleAuthor);

  const handleCopyHtml = async () => {
    await copyHtmlToClipboard(tinymce.current);
  };

  transform((data) => {
    if (data.script?.trim() === "") delete data.script;
    if (data.author_id?.trim() === "") delete data.author_id;
    return data;
  });

  const handleCreateArticle = (e: FormEvent) => {
    e.preventDefault();
    clearErrors();
    post("/gremio/noticias/criar", {
      onSuccess: () => {
        toast.success("Notícia criada com sucesso! Peça para que a revisem.", { autoClose: 3000 });
      },
      onError: (errors) => {
        if (!("error" in errors)) return;
        console.error(errors.error);
        toast.error("Não foi possível publicar a notícia. Por favor, contate um desenvolvedor.");
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
            label="Título"
            placeholder="Abracadabra"
            name="title"
            required
            validationError={errors.title}
            onInput={(e) => setData({ ...data, title: e.currentTarget.value })}
          />

          <Form.Input
            label="Descrição"
            name="description"
            placeholder="Amor, oh-na-na. Abra. Cadabra. Morta, oh-ga-ga."
            required
            validationError={errors.description}
            onInput={(e) => setData({ ...data, description: e.currentTarget.value })}
          />

          <Form.Input
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
                label="ID do autor"
                placeholder="37824kef-vduih27i2-4289328v-489uf"
                name="author_id"
                validationError={errors.author_id}
                onInput={(e) => {
                  setData({ ...data, author_id: e.currentTarget.value });
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

          <Suspense fallback={<TinyMCEEditorSkeleton />}>
            <TinyMCEEditor
              validationError={errors.content}
              onEditorChange={(content) => setData({ ...data, content })}
              editorRef={tinymce}
            />
          </Suspense>

          <div>
            <Form.Input
              asChild
              label="Script"
              placeholder={`console.log("Hello world");`}
              name="script"
              validationError={errors.script}
              onInput={(e) => {
                setData({ ...data, script: e.currentTarget.value });
              }}>
              <textarea rows={10} />
            </Form.Input>
            <p className="text-sm font-light ml-1 text-gray-800">
              Esses scripts serão executados assim que a notícia carregar. Preencha somente se
              necessário.
            </p>
          </div>

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
