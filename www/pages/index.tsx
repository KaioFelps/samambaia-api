import { colors } from "@crate/tailwind.config";
import { Link } from "@inertiajs/react";
import { Plus } from "@phosphor-icons/react/dist/ssr";

import { ArticleCard } from "@/components/articleCard";
import { Sprite } from "@/components/sprite";
import { type SharedProps } from "@/inertiaShared";
import { colorWithOpacity } from "@/lib/tailwind";
import { Article } from "@/types/article";

type HomeProps = SharedProps & {
  articles: Article[];
};

export default function Index({ articles }: HomeProps) {
  return (
    <main className="flex-1 flex flex-col gap-2">
      <section className="card w-full">
        <header className="section-header blue flex gap-3 items-center justify-between mb-3">
          <h1>
            Últimas notícias
          </h1>

          <Link
            href="/"
            style={{
              boxShadow: `
                0 2px 0 0 ${colorWithOpacity(colors.white, 25)},
                inset 0 2px 0 0 ${colorWithOpacity(colors.white, 50)}`,
            }}
            className="
              flex items-center gap-1 text-white text-sm font-medium
              border-2 border-gray-800 bg-blue-500 px-2 py-[5.5px] rounded-[6px]
              leading-tight
              "
          >
            <Plus
              size={16}
              weight="bold"
            />
            Ver mais notícias
          </Link>
        </header>

        {articles.length > 0
          ? (
            <div className="grid grid-flow-row grid-cols-2 gap-2">
              {articles.map(ArticleCard)}
            </div>
            )
          : (
            <div className="
              w-full grid place-items-center p-3 rounded-lg mx-auto bg-yellow-500/10 mt-3
              border-2 border-yellow-500/25
              "
            >
              <span className="text-yellow-900 font-bold text-lg">
                ✨ Ainda não há notícias ✨
              </span>
              <Sprite
                x={-679}
                y={-118}
                width={119}
                height={180}
              />
            </div>
            )}
      </section>
    </main>
  );
}
