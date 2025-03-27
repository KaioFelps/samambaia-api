import { useForm } from "@inertiajs/react";
import { Clipboard } from "@phosphor-icons/react/dist/ssr/Clipboard";
import { Plus } from "@phosphor-icons/react/dist/ssr/Plus";
import { Spinner } from "@phosphor-icons/react/dist/ssr/Spinner";
import { FormEvent } from "react";
import { toast } from "react-toastify";
import tinymce from "tinymce";

import { Alert } from "@/components/alert";
import Button from "@/components/button";
import { AdminDroppableIndicator } from "@/components/droppable-indicator";
import Form from "@/components/form";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import Select from "@/components/select";
import { useCanSee } from "@/hooks/useCanSee";
import { ArticleTag } from "@/types/article-tag";
import { Permission } from "@/types/auth";
import { TinyMCEEditor } from "@/ui/admin/tiny-mce-editor";

type AdminCreateArticlePageProps = {
  tags: ArticleTag[];
};

    type CreateArticleForm = {
      title: string;
      content: string;
      coverUrl: string;
      description: string;
      tagId?: number;
      authorId?: string;
    };

export default function AdminCreateArticlePage({ tags }: AdminCreateArticlePageProps) {
  const {
    data,
    setData,
    errors,
    clearErrors,
    processing,
    post,
  } = useForm<CreateArticleForm>({
    content: "",
    coverUrl: "",
    description: "",
    title: "",

  });
  const userCanPublishInNameOfOthers = useCanSee(Permission.ChangeArticleAuthor); ;

  const handleCopyHtml = async () => {
    const content = tinymce.activeEditor?.getContent();
    if (!content) {
      toast("Não há conteúdo a ser copiado.", { type: "error" });
      return;
    };

    await window.navigator.clipboard.writeText(content);
    toast("Conteúdo copiado com sucesso!", { type: "info" });
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
      onError: (error) => {
        console.error(error);
        toast("Não foi possível publicar a notícia. Por favor, contate um desenvolvedor.", {
          type: "error",
        });
      },
    });
  };

  return (
    <>
      <Head
        admin
        title="Nova notícia"
      />

      <Main admin>
        <Header.Root className="mb-8">
          <Header.Title>Criar notícia</Header.Title>
          <Header.Divisor />
        </Header.Root>

        <Form.Root
          onSubmit={handleCreateArticle}
          className="flex flex-col gap-3"
        >
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
            validationError={errors.coverUrl}
            onInput={(e) => setData({ ...data, coverUrl: e.currentTarget.value })}
          />

          {userCanPublishInNameOfOthers && (
            <div>

              <Form.Input
                admin
                label="ID do autor"
                placeholder="37824kef-vduih27i2-4289328v-489uf"
                name="author_id"
                validationError={errors.authorId}
                onInput={(e) => {
                  const value = e.currentTarget.value;
                  const authorId = value || undefined;
                  setData({ ...data, authorId });
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
            <Select.Root onValueChange={(v) => setData({ ...data, tagId: Number(v) })}>
              <Select.Trigger asChild>
                <Button
                  admin
                  variant="ghost"
                  className="w-full justify-between! text-black/50"
                >
                  <Select.Value placeholder="Escolha uma tag" />
                  <AdminDroppableIndicator />
                </Button>
              </Select.Trigger>
              <Select.Content className="bg-white p-2 w-[var(--radix-select-trigger-width)]">
                {tags.length
                  ? (

                    <Select.Viewport>
                      {tags.map((tag) => (
                        <Select.Item
                          key={"new-article-form-tag-select-input-" + tag.id}
                          label={tag.value}
                          value={tag.id.toString()}
                        />
                      ))}
                    </Select.Viewport>
                    )
                  : (
                    <Alert
                      admin
                      message="Não há tags registradas."
                      type="warning"
                      className="border-hidden rounded-lg"
                    />
                    )}
              </Select.Content>
            </Select.Root>
          </div>

          <TinyMCEEditor
            validationError={errors.content}
            onEditorChange={(content) => setData({ ...data, content })}
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
                    Publicando...
                  </>
                  )
                : (
                  <>
                    <Plus
                      size={16}
                      weight="bold"
                    />
                    Publicar
                  </>
                  )}
            </Button>
            <Button
              admin
              type="button"
              size="lg"
              onClick={handleCopyHtml}
            >
              <Clipboard
                size={16}
                weight="bold"
              />
              Copiar HTML
            </Button>
          </div>
        </Form.Root>
      </Main>
    </>
  );
}
