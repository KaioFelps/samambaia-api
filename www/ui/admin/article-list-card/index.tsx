import { useForm, usePage } from "@inertiajs/react";
import { CalendarBlankIcon } from "@phosphor-icons/react/dist/ssr/CalendarBlank";
import { EyeIcon } from "@phosphor-icons/react/dist/ssr/Eye";
import { PencilSimpleIcon } from "@phosphor-icons/react/dist/ssr/PencilSimple";
import { TagIcon } from "@phosphor-icons/react/dist/ssr/Tag";
import { UserIcon } from "@phosphor-icons/react/dist/ssr/User";
import { memo, useCallback, useState } from "react";
import { toast } from "react-toastify";

import { Chip } from "@/components/chip";
import { IconButton } from "@/components/icon-button";
import { routes } from "@/config/routes";
import type { ArticlePreview } from "@/types/article-preview";
import { Permission } from "@/types/auth";
import { can } from "@/utils/can";
import { ApproveToggleButton } from "./approve-toggle-button";
import { DeleteArticleButton } from "./delete-article-button";

type ToggleApprovedArticleForm = {
  approved: boolean;
};

export const ArticleListCard = memo(
  ({ id, tags, title, author, approved: _approved, createdAt, slug }: ArticlePreview) => {
    const [approved, setApproved] = useState(_approved);
    const auth = usePage().props.auth;
    const { patch, clearErrors, processing, data, setData } = useForm<ToggleApprovedArticleForm>({
      approved: !_approved,
    });

    const handleToggleApproved = useCallback(() => {
      clearErrors();
      const endpoint = routes.admin.articles.toggleApproved(id);

      patch(endpoint, {
        errorBag: `change-article-approved-${id}`,
        onSuccess: () => {
          setApproved(data.approved);
          setData({ approved: !data.approved });
          toast(data.approved ? "Notícia aprovada." : "Notícia desmarcada como aprovada.", {
            type: "info",
            autoClose: 3000,
          });
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

          if ("approved" in errors) {
            toast(errors.approved, { autoClose: false, type: "error" });
          }
        },
      });
    }, [id, clearErrors, patch, data, setData]);

    return (
      <article className="admin-list-card">
        <span title={title} className="flex-1/2 text-sm line-clamp-1 text-gray-800">
          {title}
        </span>

        <div className="max-w-3/5 flex gap-2 items-center justify-end">
          <div className="overflow-x-auto no-scrollbar snap-mandatory snap-x flex gap-2 items-center rounded-full">
            {tags.map((tag) => (
              <Chip
                key={`article-list-card-${id}-tag-${tag.id}`}
                icon={TagIcon}
                text={tag.value}
                size="sm"
                className="whitespace-nowrap snap-center snap-normal bg-purple-300/10"
              />
            ))}
          </div>

          <Chip icon={UserIcon} text={author.nickname} size="sm" />

          <Chip
            icon={CalendarBlankIcon}
            text={new Date(createdAt).toLocaleDateString("pt-BR")}
            size="sm"
          />

          <ApproveToggleButton
            isPublished={approved}
            isLoading={processing}
            onClick={handleToggleApproved}
          />
        </div>

        <div className="flex items-center justify-end gap-1 max-w-20 shrink-0">
          <DeleteArticleButton articleId={id} articleTitle={title} />

          {(auth?.user.id === author.id || can(auth?.permissions, Permission.UpdateArticle)) && (
            <IconButton
              admin
              size="sm"
              variant="ghost"
              theme="warn"
              icon={PencilSimpleIcon}
              asLink
              href={routes.admin.articles.edit(id)}
            />
          )}

          <IconButton
            admin
            size="sm"
            variant="ghost"
            theme="info"
            icon={EyeIcon}
            asLink
            href={routes.web.article.view(slug)}
          />
        </div>
      </article>
    );
  },
);
