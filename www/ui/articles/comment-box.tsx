import clsx from "clsx";
import { Sprite } from "@/components/sprite";
import type { Comment } from "@/types/comment";
import { Imager } from "@/utils/imager";

export function CommentBox(comment: Comment) {
  const userImage = Imager.getUserImage(comment.author.nickname, {
    size: "s",
    head_direction: "3",
  });

  return (
    <div key={`article-comment-${comment.id}`} className="border-2 border-gray-700 rounded-md flex">
      <div
        className={clsx(
          "w-10 bg-yellow-500 flex flex-col justify-end rounded-ss-sm rounded-es-sm",
          "border-r-2 border-black/25",
          "shadow-[inset_0_3px_0_0_color-mix(in_oklab,_var(--color-white)_40%,_transparent),_inset_0_-2px_0_0_color-mix(in_oklab,_var(--color-black)_10%,_transparent)]",
        )}>
        <div
          style={{
            background: `url(${userImage}) -3px -10px`,
          }}
          className="pixelated w-10 min-h-14 h-auto"
        />
      </div>
      <div
        className={clsx(
          "px-1.5 py-0.5 flex-1 bg-gray-200 rounded-se-sm rounded-ee-sm flex flex-col justify-between",
          "shadow-[inset_0_3px_0_0_var(--color-white),_inset_0_-2px_0_0_color-mix(in_oklab,_var(--color-black)_10%,_transparent)]",
        )}>
        <p className="text-sm text-black">
          <span>{comment.author.nickname}: </span>
          {comment.content}
        </p>

        <div className="flex justify-between items-center">
          <span className="text-xs text-gray-700">
            {new Date(comment.createdAt).toLocaleDateString("pt-BR")}
          </span>

          <button title="Denunciar" type="button">
            <Sprite width={16} height={18} x={-1} y={-65} />
          </button>
        </div>
      </div>
    </div>
  );
}
