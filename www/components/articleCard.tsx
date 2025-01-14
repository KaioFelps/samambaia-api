import { colors } from "@crate/tailwind.config";
import { Link } from "@inertiajs/react";

import { colorWithOpacity } from "@/lib/tailwind";
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
      className="
        group from-gray-200 to-gray-250
        bg-[linear-gradient(to_bottom,_var(--tw-gradient-from)_66%,_var(--tw-gradient-to)_66%)]
        transition-all duration-500
        rounded-lg p-2 flex gap-2 border-2 border-gray-700 shadow-white shadow-[inset_0_0_0_2px]
        "
    >
      <div
        aria-hidden
        style={{
          backgroundImage: `url("${coverUrl}")`,
          backgroundPosition: "center center",
          backgroundSize: "none",
          boxShadow: `inset 0 0 0 2px ${colorWithOpacity(colors.black, 25)}`,
        }}
        className="pixelated min-w-32 min-h-[100px] rounded-lg"
      />

      <div
        style={{ textShadow: "0px 2px var(--tw-shadow-color)" }}
        className="
            flex flex-col justify-between shadow-white group-hover:shadow-white/50
            transition-[text-shadow] duration-100
            "
      >
        <div className="block">
          <h3 className="font-bold leading-4 mb-1 line-clamp-2 text-ellipsis text-gray-800">
            <Link
              className="transition-colors duration-100 group-hover:text-blue-500 hover:underline"
              href={`/noticias/${slug}`}
            >
              {title}
            </Link>
          </h3>
          <p className="text-sm text-gray-700 leading-4 line-clamp-2 text-ellipsis cursor-default">
            {description}
          </p>
        </div>

        <div className="flex items-center gap-2 p-1 cursor-default">
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
