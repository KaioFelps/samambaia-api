import { X } from "@phosphor-icons/react/dist/ssr";
import { Close, Description, Title } from "@radix-ui/react-dialog";
import clsx from "clsx";
import { memo } from "react";

type AdminDialogHeaderProps = {
  title: string;
  description?: string;
};

export const AdminDialogHeader = memo(({ title, description }: AdminDialogHeaderProps) => {
  return (
    <header className={clsx(
      "flex items-center gap-2 justify-between p-3 bg-white",
      "shadow-sm shadow-black/5 mb-1 rounded-se-lg rounded-ss-lg",
    )}
    >
      <Title className="text-xl font-semibold text-gray-700">{title}</Title>
      {description && (
        <Description className="sr-only">{description}</Description>
      )}

      <Close className={clsx(
        "p-1 rounded-md bg-purple-300/25 text-purple-700",
        "hover:bg-purple-300/30 active:bg-purple-300/40",
      )}
      >
        <X
          size={16}
          weight="bold"
        />
      </Close>
    </header>
  );
});
