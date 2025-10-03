import { Link, router } from "@inertiajs/react";
import { GearIcon } from "@phosphor-icons/react/dist/ssr/Gear";
import { SignOutIcon } from "@phosphor-icons/react/dist/ssr/SignOut";
import { memo, useCallback, useMemo } from "react";
import Dropdown from "@/components/dropdown";
import { AdminDroppableArrow } from "@/components/droppable-arrow";
import { Sprite } from "@/components/sprite";
import { FaceGesture, Imager } from "@/utils/imager";

export const UserDropdown = memo(({ nickname }: { nickname: string }) => {
  const userHead = useMemo(
    () =>
      Imager.getUserImage(nickname, {
        headonly: "1",
        gesture: FaceGesture.smile,
        head_direction: "3",
        size: "s",
      }),
    [nickname],
  );

  const handleLogout = useCallback(() => {
    router.post("/sessions/logout");
  }, []);

  return (
    <Dropdown.Root>
      <Dropdown.Trigger className="admin-btn py-0.5">
        {nickname}
        <Sprite spriteUrl={userHead} height={28} width={28} x={-10} y={-12} />
      </Dropdown.Trigger>
      <Dropdown.Content className="admin-dropdown-content individual-focus p-1 flex flex-col">
        <Link className="admin-dropdown-content-clickable" href="/gremio">
          <GearIcon size={14} className="text-purple-300" />
          Configurações
        </Link>

        <button className="admin-dropdown-content-clickable" onClick={handleLogout} type="button">
          <SignOutIcon size={14} className="text-purple-300" />
          Logout
        </button>
        <AdminDroppableArrow component="dropdown" />
      </Dropdown.Content>
    </Dropdown.Root>
  );
});
