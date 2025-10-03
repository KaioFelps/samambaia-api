import { Link } from "@inertiajs/react";
import { CardsThreeIcon } from "@phosphor-icons/react/dist/ssr/CardsThree";
import { ImageIcon } from "@phosphor-icons/react/dist/ssr/Image";
import { PlusIcon } from "@phosphor-icons/react/dist/ssr/Plus";
import { ScrollIcon } from "@phosphor-icons/react/dist/ssr/Scroll";
import { UsersIcon } from "@phosphor-icons/react/dist/ssr/Users";
import clsx from "clsx";
import { memo } from "react";
import Dropdown from "@/components/dropdown";
import { AdminDroppableArrow } from "@/components/droppable-arrow";
import { AdminDroppableIndicator } from "@/components/droppable-indicator";

export const CreateShortcutsDropdown = memo(() => {
  const iconProps = {
    size: 14,
    className: "text-purple-500",
  };

  return (
    <Dropdown.Root>
      <Dropdown.Trigger className="group admin-btn select-none">
        <PlusIcon size={14} weight="bold" className="text-gray-700" />
        Criar...
        <AdminDroppableIndicator />
      </Dropdown.Trigger>
      <Dropdown.Content
        align="end"
        className={clsx(
          "admin-dropdown-content p-1 flex flex-col min-w-48 text-sm individual-focus",
        )}>
        <Link className="admin-dropdown-content-clickable" href="/gremio">
          <ScrollIcon {...iconProps} />
          Notícia
        </Link>

        <Link className="admin-dropdown-content-clickable" href="/gremio">
          <UsersIcon {...iconProps} />
          Usuário
        </Link>

        <Link className="admin-dropdown-content-clickable" href="/gremio">
          <ImageIcon {...iconProps} />
          Emblema
        </Link>

        <Link className="admin-dropdown-content-clickable" href="/gremio">
          <CardsThreeIcon {...iconProps} />
          Anúncio
        </Link>

        <AdminDroppableArrow component="dropdown" />
      </Dropdown.Content>
    </Dropdown.Root>
  );
});
