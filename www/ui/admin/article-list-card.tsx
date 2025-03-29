import { useForm, usePage } from "@inertiajs/react";
import { CalendarBlank } from "@phosphor-icons/react/dist/ssr/CalendarBlank";
import { CheckFat } from "@phosphor-icons/react/dist/ssr/CheckFat";
import { Eye } from "@phosphor-icons/react/dist/ssr/Eye";
import { PencilSimple } from "@phosphor-icons/react/dist/ssr/PencilSimple";
import { Spinner } from "@phosphor-icons/react/dist/ssr/Spinner";
import { Tag } from "@phosphor-icons/react/dist/ssr/Tag";
import { Trash } from "@phosphor-icons/react/dist/ssr/Trash";
import { User } from "@phosphor-icons/react/dist/ssr/User";
import clsx from "clsx";
import { memo, useCallback, useState } from "react";
import { toast } from "react-toastify";

import { Chip } from "@/components/chip";
import { IconButton } from "@/components/icon-button";
import { ArticlePreview } from "@/types/article-preview";
import { Permission } from "@/types/auth";
import { can } from "@/utils/can";

type ToggleApprovedArticleForm = {
  approved: boolean;
};

export const ArticleListCard = memo(({
  id,
  tag,
  title,
  author,
  approved: _approved,
  createdAt,
}: ArticlePreview) => {
  const [approved, setApproved] = useState(_approved);
  const auth = usePage().props.auth;
  const {
    patch,
    clearErrors,
    processing,
    data,
    setData,
  } = useForm<ToggleApprovedArticleForm>({ approved: !_approved });

  const handleToggleApproved = useCallback(() => {
    clearErrors();
    const endpoint = `/gremio/noticias/${id}/alterar-aprovado`;

    patch(endpoint, {
      errorBag: `change-article-approved-${id}`,
      onSuccess: () => {
        setApproved(data.approved);
        setData({ approved: !data.approved });
        toast(data.approved
          ? "Notícia aprovada."
          : "Notícia desmarcada como aprovada.", { type: "info", autoClose: 3000 });
      },
      onError: (errors) => {
        if ("error" in errors) {
          toast("Não foi possível aprovar este artigo. Contate um desenvolvedor.", {
            autoClose: false,
            type: "error",
          });

          console.error(errors.error);
          return;
        }

        if ("approved" in errors) { toast(errors.approved, { autoClose: false, type: "error" }); }
      },
    });
  }, [id, clearErrors, patch, data, setData]);

  return (
    <article className={clsx(
      "flex items-center gap-12 py-2 px-5 border-b border-gray-300",
      "hover:bg-white/30",
    )}
    >
      <span className="flex-1/2 text-sm line-clamp-1 text-gray-800">{title}</span>

      <div className="flex gap-2 items-center justify-end grow">
        <Chip
          icon={User}
          text={author.nickname}
          size="sm"
        />

        {tag && (
          <Chip
            icon={Tag}
            text={tag.value}
            size="sm"
          />
        )}

        <Chip
          icon={CalendarBlank}
          text={new Date(createdAt).toLocaleDateString("pt-BR")}
          size="sm"
        />

        <PublishmentCheck
          isPublished={approved}
          isLoading={processing}
          onClick={handleToggleApproved}
        />
      </div>

      <div className="flex items-center justify-end gap-1 grow max-w-20">
        {can(auth?.permissions, Permission.DeleteArticle) && (
          <IconButton
            admin
            size="sm"
            theme="danger"
            variant="ghost"
            icon={Trash}
          />
        )}

        {(auth?.user.id === author.id || can(auth?.permissions, Permission.UpdateArticle)) && (
          <IconButton
            admin
            size="sm"
            variant="ghost"
            theme="warn"
            icon={PencilSimple}
          />
        )}

        <IconButton
          admin
          size="sm"
          variant="ghost"
          theme="info"
          icon={Eye}
        />
      </div>
    </article>
  );
});

type PublishmentCheckProps = {
  isPublished: boolean;
  isLoading: boolean;
  onClick: () => void;
};

const PublishmentCheck = memo(({ isPublished, isLoading, onClick }: PublishmentCheckProps) => {
  return (
    <button
      disabled={isLoading}
      onClick={onClick}
      className={clsx(
        "p-1 rounded-full transition-all self-center outline-hidden ring-0 focus-visible:ring-4",
        !isLoading && isPublished
          ? "text-white bg-green-500 hover:bg-green-600 active:bg-green-700 ring-green-600/40"
          : "text-gray-700 bg-gray-300 hover:bg-gray-400 active:brightness-95 ring-purple-500/40",
      )}
    >
      {isLoading
        ? (
          <Spinner
            className="animate-spin"
            size={16}
            weight="bold"
          />
          )
        : (

          <CheckFat
            size={16}
            weight="fill"
          />
          )}
    </button>
  );
});
