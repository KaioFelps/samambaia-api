import { colors } from "@crate/tailwind.config";
import { Link } from "@inertiajs/react";
import { ArrowSquareOut } from "@phosphor-icons/react/dist/ssr/ArrowSquareOut";

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
        group bg-gray-200 relative transition-all will-change-[background] duration-200 delay-200
        overflow-hidden rounded-lg border-2 border-gray-700 hover:bg-gray-100
        hover:shadow-black/10 hover:shadow-md
        "
    >
      <div
        inert
        className="
          bg-gray-250 absolute bottom-0 inset-x-0 h-[40px] group-hover:h-0
          transition-[height] duration-100 delay-200 rounded-[inherit]
          "
      />

      <div
        inert
        className="shadow-white shadow-[inset_0_0_0_2px] rounded-[inherit] absolute inset-0"
      />

      <div className="p-2 flex gap-2 relative">
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
            flex flex-col justify-between w-full shadow-white group-hover:shadow-white/50
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
            <p className="
              text-sm text-gray-700 leading-4 line-clamp-2 text-ellipsis cursor-default
              group-hover:text-black transition-colors duration-150
              "
            >
              {description}
            </p>
          </div>

          <div className="relative h-[25px] w-full">
            <div className="
              w-full absolute left-0 bottom-0 flex items-center gap-2 p-1 cursor-default
              group-hover:-bottom-[120%] transition-[bottom] duration-150 delay-300
              "
            >
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

            <div className="
              w-full absolute left-[120%] bottom-0 group-hover:left-0 transition-[left] duration-150
              select-none pointer-events-none group-hover:pointer-events-auto
              delay-[350ms]
              "
            >
              <Link
                className="
                  text-nowrap flex items-center gap-2 px-2 py-1 rounded-ss-md rounded-es-md
                  w-[calc(100%_+_8px)] bg-blue-500/10 justify-between
                  [text-shadow:_none] leading-none text-sm text-blue-700
                  "
                href={`/noticias/${slug}`}
              >
                Leia mais

                <ArrowSquareOut
                  size={16}
                  weight="bold"
                />
              </Link>
            </div>
          </div>

        </div>
      </div>
    </div>
  );
}
