import { PageProps } from "@inertiajs/core/types";
import { Link } from "@inertiajs/react";
import { CaretDown } from "@phosphor-icons/react/dist/ssr/CaretDown";
import { Plus } from "@phosphor-icons/react/dist/ssr/Plus";
import clsx from "clsx";
import { ReactNode, useMemo } from "react";
import { ToastContainer } from "react-toastify";

import { Head } from "@/components/head";
import { Sprite } from "@/components/sprite";
import { appConfig } from "@/config/app";
import { FaceGesture, Imager } from "@/utils/imager";

export function AdminLayout({ children, props }: { children: ReactNode; props: PageProps }) {
  const userHead = useMemo(() => (Imager.getUserImage(props.auth!.user.nickname, {
    headonly: "1",
    gesture: FaceGesture.smile,
    head_direction: "3",
    size: "s",
  })), [props]);
  return (
    <>
      <Head title="Administração" />
      <header className={clsx(
        "bg-gray-100 px-6 py-2 flex items-center justify-between border-b border-gray-250")}
      >
        <Link href="/admin"><img src={appConfig.assets.adminLogo} /></Link>
        <div className="flex gap-2">
          <button className="btn-admin">
            <Plus
              size={14}
              weight="bold"
              className="text-gray-700"
            />
            Criar...
            <CaretDown
              size={14}
              weight="bold"
              className="text-gray-700"
            />
          </button>

          <button className="btn-admin py-0.5">
            {props.auth!.user.nickname}
            <Sprite
              spriteUrl={userHead}
              height={28}
              width={28}
              x={1}
              y={-2}
            />
          </button>
        </div>
      </header>

      <aside />
      {children}
      <footer />
      <ToastContainer />
    </>
  );
}
