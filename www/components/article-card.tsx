import { Link } from "@inertiajs/react";
import clsx from "clsx";

import { Article } from "@/types/article";

import { Sprite } from "./sprite";

export function ArticleCard({
  id,
  slug,
  title,
  author,
  coverUrl,
  createdAt,
  description,
}: Article) {
  return (
    <div
      key={"article-card-" + id}
      className={clsx(
        "group bg-gray-200 relative transition-all will-change-[background] duration-200",
        "overflow-hidden rounded-lg border-2 border-gray-700 hover:bg-gray-100",
        "hover:shadow-black/10 hover:shadow-md p-2 flex gap-2 relative",
        // before
        "before:bg-gray-250 before:absolute before:bottom-1 before:inset-x-1 before:h-[40px]",
        "hover:before:h-16 before:rounded-md before:pointer-events-none",
        "before:transition-[height] before:duration-100",
        // after
        "after:shadow-white after:shadow-[inset_0_0_0_2px] after:pointer-events-none",
        "after:rounded-[inherit] after:absolute after:inset-0",
      )}
    >
      <div
        aria-hidden
        style={{
          backgroundImage: `url("${coverUrl}")`,
          backgroundPosition: "center center",
          backgroundSize: "none",
        }}
        className={clsx(
          "pixelated min-w-32 min-h-[100px] rounded-lg relative top-0 group-hover:-top-1",
          "shadow-black/25 group-hover:shadow-black/30",
          "shadow-[inset_0_0_0_2px] group-hover:shadow-md",
          "transition-all duration-150 will-change-[shadow,_top]",
        )}
      />

      <div
        style={{ textShadow: "0px 2px var(--tw-shadow-color)" }}
        className="
            flex flex-col justify-between w-full shadow-white group-hover:shadow-white/50
            transition-[text-shadow] duration-100 relative
            "
      >
        <div className="block">
          <h3 className="font-bold leading-4 mb-1 line-clamp-2 text-ellipsis text-gray-800">
            <Link
              className="transition-colors group-hover:text-blue-500 hover:!text-blue-700"
              href={`/noticias/${slug}`}
            >
              {title}
            </Link>
          </h3>
          <p className="
              text-sm text-gray-700 leading-4 line-clamp-2 text-ellipsis cursor-default
              group-hover:text-black transition-colors duration-150
              "
          >
            {description}
          </p>
        </div>

        <div className="w-full flex items-center gap-2 p-1 cursor-default">
          <div className="flex gap-1 items-center text-sm text-gray-800 leading-none">
            <Sprite
              x={-139}
              y={-64}
              width={16}
              height={16}
            />
            {author.nickname}
          </div>

          <div className="flex gap-1 items-center text-sm text-gray-800 leading-none">
            <Sprite
              x={-128}
              y={-96}
              width={16}
              height={17}
            />
            {new Date(createdAt).toLocaleDateString()}
          </div>
        </div>
      </div>
    </div>
  );
}
