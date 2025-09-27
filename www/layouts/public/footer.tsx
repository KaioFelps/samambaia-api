import { Link } from "@inertiajs/react";
import { Code } from "@phosphor-icons/react/dist/ssr/Code";
import clsx from "clsx";
import { memo } from "react";

import { Sprite } from "@/components/sprite";
import { appConfig } from "@/config/app";
import { useCanSee } from "@/hooks/useCanSee";
import { Permission } from "@/types/auth";

export const Footer = memo(() => {
  const canSeeTheDashboard = useCanSee(Permission.AccessDashboard);

  return (
    <footer
      style={{
        boxShadow: `
          inset 0 2px 0 0 color-mix(in oklab, var(--color-black) 25%, transparent),
          inset 0 4px 0 0 var(--color-white),
          0 2px 0 0 color-mix(in oklab, var(--color-black) 15%, transparent)
        `,
      }}
      className="mt-2 bg-gray-200 h-fit">
      <header className="main-screen-centralized flex items-center justify-between py-1.5">
        <span className="text-gray-700 font-medium flex gap-2 items-center">
          <Code size={16} weight="bold" /> Desenvolvido por{" "}
          <span className="text-yellow-900">Floricultor</span>
        </span>

        <div className="flex gap-1">
          <Link href="" target="_blank">
            <Sprite x={-320} y={0} width={33} height={33} />
          </Link>
          <Link href="" target="_blank">
            <Sprite x={-357} y={0} width={33} height={33} />
          </Link>
          <Link href="" target="_blank">
            <Sprite x={-394} y={0} width={33} height={33} />
          </Link>
        </div>
      </header>

      <div className="w-full h-0.5 bg-black/15 shadow-white shadow-[0_2px_0_0]" />

      <div className="flex items-stretch main-screen-centralized">
        <div className="flex-1 flex max-w-[441px] gap-12 items-center">
          <Sprite x={-721} y={-2} width={78} height={78} />

          <p className="text-balance text-gray-700 text-sm">
            <span className="block mb-1">Copyright {appConfig.appName} 2025</span>
            Somos um fã-site oficial do {appConfig.hostHotel.name}. Não somos aprovados,
            patrocinados, filiados nem reconhecidos pela Sulake.
          </p>
        </div>

        <div className="h-auto w-0.5 bg-black/15 shadow-white shadow-[2px_1px_0_0] mx-[72px]" />

        <div
          className={clsx(
            "flex-1 flex items-start gap-2 py-6 prose-a:font-medium prose-a:hover:text-purple-700",
            "prose-a:hover:underline prose-a:transition-all prose-a:duration-300",
            "prose-a:ml-0 prose-a:hover:ml-2 prose-a:w-fit prose-a:decoration-purple-300",
          )}>
          <div className="flex-1 flex flex-col">
            <span className="text-purple-700/70 font-bold text-sm mb-2">Cosmic</span>
            <div className="flex flex-col">
              <Link href="/">Página inicial</Link>
              <Link href="">Sobre o {appConfig.appName}</Link>
              <Link href="">Equipe Cósmic</Link>
              <Link href="">Vagas na Equipe</Link>
              {canSeeTheDashboard && <Link href="/gremio">Painel do Grêmio</Link>}
            </div>
          </div>

          <div className="flex-1 flex flex-col">
            <span className="text-purple-700/70 font-bold text-sm mb-2">Jornalismo</span>
            <div className="flex flex-col">
              <Link href="">Arquivo de notícias</Link>
              <Link href="">Histórico de campanhas</Link>
              <Link href="">Promoções ativas</Link>
            </div>
          </div>

          <div className="flex-1 flex flex-col">
            <span className="text-purple-700/70 font-bold text-sm mb-2">
              {appConfig.hostHotel.name}
            </span>
            <div className="flex flex-col">
              <a href={appConfig.hostHotel.baseUrl} target="_blank">
                Jogar {appConfig.hostHotel.shortName}
              </a>
              <Link href="">Promoções ativas</Link>
              <Link href="">Campanhas ativas</Link>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
});
