import { PageProps } from "@inertiajs/core/types";
import { Link, router } from "@inertiajs/react";
import { CardsThree } from "@phosphor-icons/react/dist/ssr/CardsThree";
import { Gear } from "@phosphor-icons/react/dist/ssr/Gear";
import { Image } from "@phosphor-icons/react/dist/ssr/Image";
import { Plus } from "@phosphor-icons/react/dist/ssr/Plus";
import { Scroll } from "@phosphor-icons/react/dist/ssr/Scroll";
import { SignOut } from "@phosphor-icons/react/dist/ssr/SignOut";
import { Users } from "@phosphor-icons/react/dist/ssr/Users";
import clsx from "clsx";
import { memo, ReactNode, useCallback, useMemo } from "react";
import { ToastContainer } from "react-toastify";

import Dropdown from "@/components/dropdown";
import { AdminDroppableArrow } from "@/components/droppable-arrow";
import { AdminDroppableIndicator } from "@/components/droppable-indicator";
import { Head } from "@/components/head";
import { Sprite } from "@/components/sprite";
import { appConfig } from "@/config/app";
import { SidebarMenu } from "@/ui/admin/sidebar";
import { FaceGesture, Imager } from "@/utils/imager";

export function AdminLayout({ children, props }: { children: ReactNode; props: PageProps }) {
  return (
    <>
      <Head title="Administração" />
      <div className="admin-layout">
        <header className={clsx(
          "[grid-area:_header]",
          "bg-gray-100 px-6 py-2 flex items-center justify-between border-b border-gray-250",
        )}
        >
          <Link href="/gremio"><img src={appConfig.assets.adminLogo} /></Link>
          <div className="flex gap-2">
            <CreateShortcutsDropdown />
            <UserDropdown nickname={props.auth!.user.nickname} />
          </div>
        </header>

        <SidebarMenu />
        {children}

        <footer className={clsx(
          "[grid-area:_footer] h-10 bg-black/5 flex items-center justify-between gap-3",
          "p-6 py-12 rounded-lg mx-6 mb-6",
        )}
        >
          <div className="font-light text-gray-800 leading-4">
            <span>Cosmic CMS 2.0. Todos os direitos reservados.</span>
            <br />
            <span>Desenvolvido por <strong>Floricultor</strong></span>
          </div>

          <div className={clsx(
            "flex items-center gap-3",
            "prose-a:text-blue-500 hover:prose-a:underline active:prose-a:text-blue-700",
            "prose-a:cursor-default active:prose-a:cursor-pointer",
          )}
          >
            <a
              href=""
              target="_blank"
            >
              Discord da equipe
            </a>

            <a
              href=""
              target="_blank"
            >
              Twitter do fã-site
            </a>

            <a
              href=""
              target="_blank"
            >
              Home do fã-site
            </a>
          </div>
        </footer>
      </div>
      <ToastContainer />
    </>
  );
}

const CreateShortcutsDropdown = memo(() => {
  const iconProps = {
    size: 14,
    className: "text-purple-500",
  };

  return (
    <Dropdown.Root>
      <Dropdown.Trigger className="group admin-btn select-none">
        <Plus
          size={14}
          weight="bold"
          className="text-gray-700"
        />
        Criar...

        <AdminDroppableIndicator />
      </Dropdown.Trigger>
      <Dropdown.Content
        align="end"
        className={clsx(
          "admin-dropdown-content p-1 flex flex-col min-w-48 text-sm individual-focus",
        )}
      >
        <Link
          className="admin-dropdown-content-clickable"
          href="/gremio"
        >
          <Scroll {...iconProps} />
          Notícia
        </Link>

        <Link
          className="admin-dropdown-content-clickable"
          href="/gremio"
        >
          <Users {...iconProps} />
          Usuário
        </Link>

        <Link
          className="admin-dropdown-content-clickable"
          href="/gremio"
        >
          <Image {...iconProps} />
          Emblema
        </Link>

        <Link
          className="admin-dropdown-content-clickable"
          href="/gremio"
        >
          <CardsThree {...iconProps} />
          Anúncio
        </Link>

        <AdminDroppableArrow component="dropdown" />
      </Dropdown.Content>
    </Dropdown.Root>
  );
});

const UserDropdown = memo(({ nickname }:{ nickname: string }) => {
  const userHead = useMemo(() => (Imager.getUserImage(nickname, {
    headonly: "1",
    gesture: FaceGesture.smile,
    head_direction: "3",
    size: "s",
  })), [nickname]);

  const handleLogout = useCallback(() => {
    router.post("/sessions/logout");
  }, []);

  return (
    <Dropdown.Root>
      <Dropdown.Trigger className="admin-btn py-0.5">
        {nickname}
        <Sprite
          spriteUrl={userHead}
          height={28}
          width={28}
          x={1}
          y={-2}
        />
      </Dropdown.Trigger>
      <Dropdown.Content className="admin-dropdown-content individual-focus p-1 flex flex-col">
        <Link
          className="admin-dropdown-content-clickable"
          href="/gremio"
        >
          <Gear
            size={14}
            className="text-purple-300"
          />
          Configurações
        </Link>

        <button
          className="admin-dropdown-content-clickable"
          onClick={handleLogout}
          type="button"
        >
          <SignOut
            size={14}
            className="text-purple-300"
          />
          Logout
        </button>
        <AdminDroppableArrow component="dropdown" />
      </Dropdown.Content>
    </Dropdown.Root>
  );
});
