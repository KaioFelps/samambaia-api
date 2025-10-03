import type { PageProps } from "@inertiajs/core/types";
import { Link } from "@inertiajs/react";
import clsx from "clsx";
import type { ReactNode } from "react";
import { ToastContainer } from "react-toastify";

import { Head } from "@/components/head";
import { appConfig } from "@/config/app";
import { SidebarMenu } from "@/ui/admin/sidebar";
import { ProvidersWrapper } from "../providers-wrapper";
import { CreateShortcutsDropdown } from "./create-shortcuts-dropdown";
import { UserDropdown } from "./user-dropdown";

export function AdminLayout({ children, props }: { children: ReactNode; props: PageProps }) {
  return (
    <>
      <Head title="Administração" />
      <ProvidersWrapper>
        <div className="admin-layout">
          <header
            className={clsx(
              "[grid-area:_header]",
              "bg-gray-100 px-6 py-2 flex items-center justify-between border-b border-gray-250",
            )}>
            <Link href="/gremio">
              <img src={appConfig.assets.adminLogo} alt="Cosmic" className="pixelated" />
            </Link>
            <div className="flex gap-2">
              <CreateShortcutsDropdown />
              <UserDropdown nickname={props.auth!.user.nickname} />
            </div>
          </header>

          <SidebarMenu />
          {children}

          <footer
            className={clsx(
              "[grid-area:_footer] h-10 bg-black/5 flex items-center justify-between gap-3",
              "p-6 py-12 rounded-lg mx-6 mb-6",
            )}>
            <div className="font-light text-gray-800 leading-4">
              <span>Cosmic CMS 2.0. Todos os direitos reservados.</span>
              <br />
              <span>
                Desenvolvido por <strong>Floricultor</strong>
              </span>
            </div>

            <div
              className={clsx(
                "flex items-center gap-3",
                "prose-a:text-blue-500 prose-a:hover:underline prose-a:active:text-blue-700",
                "prose-a:cursor-default prose-a:active:cursor-pointer",
              )}>
              <a href="/" target="_blank" rel="noopener">
                Discord da equipe
              </a>

              <a href="/" target="_blank" rel="noopener">
                Twitter do fã-site
              </a>

              <Link href="/">Home do fã-site</Link>
            </div>
          </footer>
        </div>
        <ToastContainer />
      </ProvidersWrapper>
    </>
  );
}
