import * as Dialog from "@radix-ui/react-dialog";
import clsx from "clsx";

import { Sprite } from "@/components/sprite";

export type DialogHeaderProps = {
  title: string;
  description?: string;
  className?: string;
};

export function DialogHeader({ title, description, className }: DialogHeaderProps) {
  return (
    <header className="section-header purple flex items-center justify-between gap-3 mb-3">
      <Dialog.Title>{title}</Dialog.Title>
      <Dialog.Description className="sr-only">{description}</Dialog.Description>

      <Dialog.Close
        title="Fechar diálogo"
        aria-describedby="Fechar a janela pop-up"
        autoFocus={false}
        className={clsx(
          "transition-all duration-150 will-change-[shadow]",
          "outline-none ring-0 focus-visible:ring-4 ring-purple-100 rounded-md",
          "hover:brightness-90 active:brightness-75",
          className && className,
        )}
      >
        <Sprite
          x={-160}
          y={-63}
          width={20}
          height={20}
        />
      </Dialog.Close>
    </header>
  );
}
