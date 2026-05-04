import { usePage } from "@inertiajs/react";
import { memo } from "react";
import type { ArticleTag } from "@/types/article-tag";
import { Permission } from "@/types/auth";
import { can } from "@/utils/can";
import { DeleteArticleTagButton } from "./delete-article-tag-button";
import { UpdateArticleTagButton } from "./update-article-tag-button";

type Props = { tag: ArticleTag };

export const ArticleTagListCard = memo(({ tag }: Props) => {
  const auth = usePage().props.auth;
  const userCanEditTag = can(auth?.permissions, Permission.UpdateArticleTag);
  const userCanDeleteTag = can(auth?.permissions, Permission.DeleteArticleTag);

  return (
    <li className="admin-list-card list-none">
      <span className="flex-1/2 text-sm line-clamp-1 text-gray-800">{tag.value}</span>
      <div className="flex items-center justify-end gap-1 grow max-w-20">
        {userCanDeleteTag && <DeleteArticleTagButton tagId={tag.id} tagValue={tag.value} />}
        {userCanEditTag && <UpdateArticleTagButton tag={tag} />}
      </div>
    </li>
  );
});
