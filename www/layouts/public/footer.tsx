import { colors } from "@crate/tailwind.config";
import { Link } from "@inertiajs/react";
import { Code } from "@phosphor-icons/react/dist/ssr/Code";
import { memo } from "react";

import { Sprite } from "@/components/sprite";
import { colorWithOpacity } from "@/lib/tailwind";

export const Footer = memo(() => {
  return (
    <footer
      style={{
        boxShadow: `
          inset 0 2px 0 0 ${colorWithOpacity(colors.black, 25)},
          inset 0 4px 0 0 ${colorWithOpacity(colors.white)},
          0 2px 0 0 ${colorWithOpacity(colors.black, 15)}
        `,
      }}
      className="mt-2 bg-gray-200 h-fit"
    >
      <header className="main-screen-centralized flex items-center justify-between py-1.5">
        <span className="text-gray-700 font-medium flex gap-2 items-center">
          <Code
            size={16}
            weight="bold"
          />

          {" "}Desenvolvido por <span className="text-yellow-900">Floricultor</span>
        </span>

        <div className="flex gap-1">
          <Link
            href=""
            target="_blank"
          >
            <Sprite
              x={-320}
              y={0}
              width={33}
              height={33}
            />
          </Link>
          <Link
            href=""
            target="_blank"
          >
            <Sprite
              x={-357}
              y={0}
              width={33}
              height={33}
            />
          </Link>
          <Link
            href=""
            target="_blank"
          >
            <Sprite
              x={-394}
              y={0}
              width={33}
              height={33}
            />
          </Link>
        </div>
      </header>

      <div className="w-full h-0.5 bg-black/15 shadow-white shadow-[0_2px_0_0]" />

      <div className="flex items-stretch main-screen-centralized">
        <div className="flex-1 flex max-w-[441px] gap-12 items-center">
          <Sprite
            x={-721}
            y={-2}
            width={78}
            height={78}
          />

          <p className="text-balance text-gray-700 text-sm">
            <span className="block mb-1">Copyright Live Cosmic 2025</span>
            Somos um fã-site oficial do Habblive Hotel, não somos aprovados, patrocinados,
            filiados nem reconhecidos pela Sulake.
          </p>
        </div>

        <div className="h-auto w-0.5 bg-black/15 shadow-white shadow-[2px_1px_0_0] mx-[72px]" />

        <div className="
            flex-1 flex items-start gap-2 py-6 prose-a:font-medium hover:prose-a:text-purple-700
            active:prose-a:underline prose-a:transition-colors prose-a:duration-75
        "
        >
          <div className="flex-1 flex flex-col">
            <span className="text-purple-500/70 font-bold text-sm mb-2">Cosmic</span>
            <div className="flex flex-col">
              <Link href="/">Página inicial</Link>
              <Link href="">Sobre o Live Cosmic</Link>
              <Link href="">Equipe Cósmic</Link>
              <Link href="">Vagas na Equipe</Link>
            </div>
          </div>

          <div className="flex-1 flex flex-col">
            <span className="text-purple-500/70 font-bold text-sm mb-2">Jornalismo</span>
            <div className="flex flex-col">
              <Link href="">Arquivo de notícias</Link>
              <Link href="">Histórico de campanhas</Link>
              <Link href="">Promoções ativas</Link>
            </div>
          </div>

          <div className="flex-1 flex flex-col">
            <span className="text-purple-500/70 font-bold text-sm mb-2">Habblive</span>
            <div className="flex flex-col">
              <Link href="">Jogar Habblive</Link>
              <Link href="">Promoções ativas</Link>
              <Link href="">Campanhas ativas</Link>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
});
