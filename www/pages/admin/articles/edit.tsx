import { PageProps } from "@inertiajs/core/types";
import { useForm } from "@inertiajs/react";
import { Plus } from "@phosphor-icons/react/dist/ssr/Plus";
import { Spinner } from "@phosphor-icons/react/dist/ssr/Spinner";
import { FormEvent, useMemo } from "react";
import { toast } from "react-toastify";

import { MultiSelect, SelectOption, SelectOptions } from "@/components/admin/multiselect";
import { Alert } from "@/components/alert";
import Button from "@/components/button";
import Form from "@/components/form";
import { ValidationErrorSpan } from "@/components/form/validation-error-alert";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import { useCanSee } from "@/hooks/useCanSee";
import { Article } from "@/types/article";
import { ArticleTag } from "@/types/article-tag";
import { Permission } from "@/types/auth";
import { TinyMCEEditor } from "@/ui/admin/tiny-mce-editor";

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
};

function setTagsIfChanged(
  currentTags: ArticleTag[],
  newTags: SelectOptions,
  data: EditArticleForm,
  setData:(_data: EditArticleForm) => void,
) {
  const currentTagsSet = new Set(currentTags.map(tag => tag.id));
  const newTagsSet = new Set(newTags.map(tag => tag.value));

  const hasChanges = newTagsSet.difference(currentTagsSet).size !== 0;

  setData({
    ...data,
    tags: hasChanges
      ? newTagsSet.values().map(tagId => Number(tagId)).toArray()
      : undefined,
  });
}

export default function AdminEditArticlePage({ article, tags, flash }: AdminEditArticlePageProps) {
  const { data, setData, errors, clearErrors, processing, put } = useForm<EditArticleForm>({});

  const userCanPublishInNameOfOthers = useCanSee(Permission.ChangeArticleAuthor);

  const tagSelectOptions = useMemo(
    () => tags.map(tag => ({ label: tag.value, value: tag.id.toString() } satisfies SelectOption)),
    [tags]);

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
        <Head
          admin
          title="Editar notícia"
        />
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
      <Head
        admin
        title="Editar notícia"
      />
      <Main admin>
        <Header.Root className="mb-8">
          <Header.Title>Editando notícia</Header.Title>
          <Header.Divisor />
        </Header.Root>

        {"editArticleSuccess" in flash && (
          <Alert
            admin
            type="success"
            message={flash.editArticleSuccess}
            className="mb-6"
          />
        )}

        <Form.Root
          onSubmit={handleEditArticle}
          className="flex flex-col gap-3"
        >
          <Form.Input
            admin
            label="Título"
            placeholder="Abracadabra"
            name="title"
            required
            defaultValue={article.title}
            validationError={errors.title}
            onInput={(e) => {
              const value = e.currentTarget.value;
              setData({
                ...data,
                title: value === article.title
                  ? undefined
                  : value,
              });
            }}
          />

          <Form.Input
            admin
            label="Descrição"
            name="description"
            placeholder="Amor, oh-na-na. Abra. Cadabra. Morta, oh-ga-ga."
            required
            defaultValue={article.description}
            validationError={errors.description}
            onInput={(e) => {
              const value = e.currentTarget.value;
              setData({
                ...data,
                description: value === article.description
                  ? undefined
                  : value,
              });
            }}
          />

          <Form.Input
            admin
            label="Topstory"
            placeholder="https://i.imgur.com/..."
            name="cover_url"
            required
            defaultValue={article.coverUrl}
            validationError={errors.cover_url}
            onInput={(e) => {
              const value = e.currentTarget.value;

              setData({
                ...data,
                cover_url: value === article.coverUrl
                  ? undefined
                  : value,
              });
            }}
          />

          {userCanPublishInNameOfOthers && (
            <div>

              <Form.Input
                admin
                label="ID do autor"
                placeholder="37824kef-vduih27i2-4289328v-489uf"
                name="author_id"
                defaultValue={article.authorId}
                validationError={errors.author_id}
                onInput={(e) => {
                  const value = e.currentTarget.value;
                  setData({
                    ...data,
                    author_id: value === article.authorId
                      ? undefined
                      : value,
                  });
                }}
              />
              <p className="text-sm font-light ml-1 text-gray-800">
                Ao preencher esse campo, a notícia será publicada no usuário com o ID
                especificado.
              </p>
            </div>
          )}

          <div>
            <label className="block text-sm mb-1 ml-1">
              Tag/Categoria
            </label>
            <ValidationErrorSpan validationError={errors.tags} />
            <MultiSelect
              options={tagSelectOptions}
              setValues={(value) => setTagsIfChanged(article.tags, value, data, setData)}
            />
          </div>

          <TinyMCEEditor
            validationError={errors.content}
            onEditorChange={(_content) => {
              setData({
                ...data,
                content: _content === article.content
                  ? undefined
                  : _content,
              });
            }}
            initialValue={article.content}
          />

          <div className="mt-3 flex items-center gap-1.5">

            <Button
              admin
              variant="default"
              theme="success"
              size="lg"
              disabled={processing}
            >
              {processing
                ? (
                  <>
                    <Spinner
                      size={16}
                      weight="bold"
                      className="animate-spin"
                    />
                    Salvando...
                  </>
                  )
                : (
                  <>
                    <Plus
                      size={16}
                      weight="bold"
                    />
                    Salvar
                  </>
                  )}
            </Button>
            <Button
              admin
              asLink
              type="button"
              size="lg"
              variant="ghost"
              href="/gremio/noticias"
            >
              Cancelar
            </Button>
          </div>
        </Form.Root>
      </Main>
    </>
  );
}
