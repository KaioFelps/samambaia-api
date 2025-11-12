import type { PageProps } from "@inertiajs/core/types";
import { useForm } from "@inertiajs/react";
import { ClipboardIcon } from "@phosphor-icons/react/dist/ssr/Clipboard";
import { PlusIcon } from "@phosphor-icons/react/dist/ssr/Plus";
import { SpinnerIcon } from "@phosphor-icons/react/dist/ssr/Spinner";
import { type FormEvent, lazy, Suspense, useMemo, useRef } from "react";
import { toast } from "react-toastify";
import type { Editor } from "tinymce";
import MultiSelect, { type SelectOption, type SelectOptions } from "@/components/admin/multiselect";
import { Alert } from "@/components/alert";
import Button from "@/components/button";
import Form from "@/components/form/admin-form";
import { ValidationErrorSpan } from "@/components/form/validation-error-alert";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import { useCanSee } from "@/hooks/useCanSee";
import type { Article } from "@/types/article";
import type { ArticleTag } from "@/types/article-tag";
import { Permission } from "@/types/auth";
import { TinyMCEEditorSkeleton } from "@/ui/admin/tiny-mce-editor/skeleton";
import { contentsAreEquivalent, decodeQuotes, encodeQuotes } from "@/utils/quotes";
import { copyHtmlToClipboard } from "./shared";

const TinyMCEEditor = lazy(() => import("@/ui/admin/tiny-mce-editor"));

type AdminEditArticlePageProps = PageProps & {
  article: Article | null;
  tags: ArticleTag[];
};

type EditArticleForm = {
  title?: string;
  content?: string;
  cover_url?: string;
  description?: string;
  tags?: number[];
  author_id?: string;
  script?: string | null;
};

function haveTagsChanged(newTagsIDs: number[], currentTagsIDs: number[]): boolean {
  return (
    newTagsIDs.length !== currentTagsIDs.length ||
    !newTagsIDs.every((value) => currentTagsIDs.includes(value))
  );
}

function setTagsIfChanged(
  currentTags: ArticleTag[],
  newTags: SelectOptions,
  data: EditArticleForm,
  setData: (_data: EditArticleForm) => void,
) {
  const currentTagsIDs = currentTags.map((tag) => tag.id);
  // Remove duplicated IDs
  const newTagsIDs = new Set(newTags.map((tag) => Number(tag.value))).values().toArray();
  const hasChanged = haveTagsChanged(newTagsIDs, currentTagsIDs);
  setData({ ...data, tags: hasChanged ? newTagsIDs : undefined });
}

export default function AdminEditArticlePage({ article, tags, flash }: AdminEditArticlePageProps) {
  const tinymce = useRef<Editor>(null);
  const { data, setData, errors, clearErrors, processing, put, transform } =
    useForm<EditArticleForm>({
      author_id: article?.authorId,
      content: article ? decodeQuotes(article.content) : "",
      cover_url: article?.coverUrl,
      description: article?.description,
      script: article?.script ? decodeQuotes(article.script) : "",
      tags: article?.tags.map((tag) => tag.id),
      title: article?.title,
    });

  const userCanPublishInNameOfOthers = useCanSee(Permission.ChangeArticleAuthor);

  const tagSelectOptions = useMemo(
    () =>
      tags.map((tag) => ({ label: tag.value, value: tag.id.toString() }) satisfies SelectOption),
    [tags],
  );

  const articleCurrentTags = useMemo(
    () =>
      article?.tags.map(
        (tag) => ({ label: tag.value, value: tag.id.toString() }) satisfies SelectOption,
      ),
    [article],
  );

  const handleCopyHtml = async () => {
    await copyHtmlToClipboard(tinymce.current);
  };

  transform((_data) => {
    const data = { ..._data };
    if (!article) return data;

    if (data.title === article.title) delete data.title;
    if (data.description === article.description) delete data.description;
    if (data.cover_url === article.coverUrl) delete data.cover_url;
    if (data.author_id === article.authorId) delete data.author_id;

    if (contentsAreEquivalent(data.content, article.content)) delete data.content;
    else if (data.content) data.content = encodeQuotes(data.content);

    if (data.script && contentsAreEquivalent(data.script, article.script)) delete data.script;
    else if (!data.script?.trim()) data.script = null;
    else data.script = encodeQuotes(data.script);

    const articleTagsIDs = article.tags.map((tag) => tag.id);
    if (data.tags && !haveTagsChanged(data.tags, articleTagsIDs)) delete data.tags;

    return data;
  });

  function handleEditArticle(e: FormEvent) {
    e.preventDefault();
    clearErrors();

    if (!article) return;
    const endpoint = `/gremio/noticias/${article.id}/atualizar`;

    put(endpoint, {
      onSuccess: () => {
        toast("Notícia editada.", { type: "success" });
      },
      onError: (errors) => {
        if ("error" in errors) {
          console.error(errors.error);

          toast("Não foi possível atualizar a notícia neste momento. Contate um desenvolvedor.", {
            type: "error",
          });
        }
      },
    });
  }

  if (!article) {
    return (
      <>
        <Head admin title="Editar notícia" />
        <Main admin>
          <Header.Root className="mb-8">
            <Header.Title>Notícia não encontrada</Header.Title>
            <Header.Divisor />
          </Header.Root>

          <Alert
            admin
            type="warning"
            message={
              "A notícia especificada não foi encontrada ou talvez " +
              "você não possa visualizá-la."
            }
          />
        </Main>
      </>
    );
  }

  return (
    <>
      <Head admin title="Editar notícia" />
      <Main admin>
        <Header.Root className="mb-8">
          <Header.Title>Editando notícia</Header.Title>
          <Header.Divisor />
        </Header.Root>

        {"editArticleSuccess" in flash && (
          <Alert admin type="success" message={flash.editArticleSuccess} className="mb-6" />
        )}

        <Form.Root onSubmit={handleEditArticle} className="flex flex-col gap-3">
          <Form.Input
            label="Título"
            placeholder="Abracadabra"
            name="title"
            required
            defaultValue={article.title}
            validationError={errors.title}
            onInput={(e) => setData({ ...data, title: e.currentTarget.value })}
          />

          <Form.Input
            label="Descrição"
            name="description"
            placeholder="Amor, oh-na-na. Abra. Cadabra. Morta, oh-ga-ga."
            required
            defaultValue={article.description}
            validationError={errors.description}
            onInput={(e) => setData({ ...data, description: e.currentTarget.value })}
          />

          <Form.Input
            label="Topstory"
            placeholder="https://i.imgur.com/..."
            name="cover_url"
            required
            defaultValue={article.coverUrl}
            validationError={errors.cover_url}
            onInput={(e) => setData({ ...data, cover_url: e.currentTarget.value })}
          />

          {userCanPublishInNameOfOthers && (
            <div>
              <Form.Input
                label="ID do autor"
                placeholder="37824kef-vduih27i2-4289328v-489uf"
                name="author_id"
                defaultValue={article.authorId}
                validationError={errors.author_id}
                onInput={(e) => setData({ ...data, author_id: e.currentTarget.value })}
              />
              <p className="text-sm font-light ml-1 text-gray-800">
                Ao preencher esse campo, a notícia será publicada no usuário com o ID especificado.
              </p>
            </div>
          )}

          <div>
            <label htmlFor="edit-article-tags-select" className="block text-sm mb-1 ml-1">
              Tag/Categoria
            </label>
            <ValidationErrorSpan validationError={errors.tags} />
            <MultiSelect
              id="edit-article-tags-select"
              defaultOptions={articleCurrentTags}
              options={tagSelectOptions}
              setValues={(value) => setTagsIfChanged(article.tags, value, data, setData)}
            />
          </div>

          <Suspense fallback={<TinyMCEEditorSkeleton />}>
            <TinyMCEEditor
              validationError={errors.content}
              onEditorChange={(content) => setData({ ...data, content })}
              initialValue={data.content}
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
              defaultValue={data.script ?? ""}
              onInput={(e) => setData({ ...data, script: e.currentTarget.value })}>
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
                  Salvando...
                </>
              ) : (
                <>
                  <PlusIcon size={16} weight="bold" />
                  Salvar
                </>
              )}
            </Button>
            <Button admin type="button" size="lg" onClick={handleCopyHtml}>
              <ClipboardIcon size={16} weight="bold" />
              Copiar HTML
            </Button>
            <Button admin asLink type="button" size="lg" variant="ghost" href="/gremio/noticias">
              Cancelar
            </Button>
          </div>
        </Form.Root>
      </Main>
    </>
  );
}
