import { usePage } from "@inertiajs/react";
import { PencilSimpleIcon } from "@phosphor-icons/react/dist/ssr/PencilSimple";
import { memo } from "react";
import { AdminIconButton } from "@/components/icon-button/admin-icon-button";
import { routes } from "@/config/routes";
import type { ArticleTag } from "@/types/article-tag";
import { Permission } from "@/types/auth";
import { can } from "@/utils/can";
import { DeleteArticleTagButton } from "./delete-article-tag-button";

type Props = ArticleTag;

export const ArticleTagListCard = memo(({ id, value }: Props) => {
  const auth = usePage().props.auth;
  const userCanEditTag = can(auth?.permissions, Permission.UpdateArticleTag);
  const userCanDeleteTag = can(auth?.permissions, Permission.DeleteArticleTag);

  return (
    <li className="admin-list-card list-none">
      <span className="flex-1/2 text-sm line-clamp-1 text-gray-800">{value}</span>
      <div className="flex items-center justify-end gap-1 grow max-w-20">
        {userCanDeleteTag && <DeleteArticleTagButton tagId={id} tagValue={value} />}

        {userCanEditTag && (
          <AdminIconButton
            size="sm"
            variant="ghost"
            theme="warn"
            icon={PencilSimpleIcon}
            asLink
            href={routes.admin.tags.edit(id)}
          />
        )}
      </div>
    </li>
  );
});
