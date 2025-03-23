import { usePage } from "@inertiajs/react";
import { CalendarBlank } from "@phosphor-icons/react/dist/ssr/CalendarBlank";
import { CheckFat } from "@phosphor-icons/react/dist/ssr/CheckFat";
import { Eye } from "@phosphor-icons/react/dist/ssr/Eye";
import { PencilSimple } from "@phosphor-icons/react/dist/ssr/PencilSimple";
import { Tag } from "@phosphor-icons/react/dist/ssr/Tag";
import { Trash } from "@phosphor-icons/react/dist/ssr/Trash";
import { User } from "@phosphor-icons/react/dist/ssr/User";
import clsx from "clsx";
import { memo } from "react";

import { Chip } from "@/components/chip";
import { IconButton } from "@/components/icon-button";
import { ArticlePreview } from "@/types/article-preview";
import { Permission } from "@/types/auth";
import { can } from "@/utils/can";

export const ArticleListCard = memo(({
  tag,
  title,
  author,
  approved,
  createdAt,
}: ArticlePreview) => {
  const auth = usePage().props.auth;

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

        <PublishmentCheck isPublished={approved} />
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
};

const PublishmentCheck = memo(({ isPublished }: PublishmentCheckProps) => {
  return (
    <button className={clsx(
      "p-1 rounded-full transition-all self-center outline-hidden ring-0 focus-visible:ring-4",
      isPublished
        ? "text-white bg-green-500 hover:bg-green-600 active:bg-green-700 ring-green-600/40"
        : "text-gray-700 bg-gray-300 hover:bg-gray-400 active:brightness-95 ring-purple-500/40",
    )}
    >
      <CheckFat
        size={16}
        weight="fill"
      />
    </button>
  );
});
