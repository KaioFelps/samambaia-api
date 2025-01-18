import * as Dialog from "@radix-ui/react-dialog";
import clsx from "clsx";
import { JSX } from "react";

import { Sprite } from "@/components/sprite";

export type DialogHeaderProps = {
  title: string;
  description?: string;
  className?: string;
  noCloseButton?: boolean;
  customClose?: JSX.Element;
};

export function DialogHeader({
  title,
  description,
  className,
  noCloseButton = false,
  customClose: CustomClose,
}: DialogHeaderProps) {
  const CloseButton = noCloseButton
    ? null
    : CustomClose ?? (
      <Dialog.Close
        title="Fechar diálogo"
        aria-describedby="Fechar a janela pop-up"
        autoFocus={false}
        className={clsx(
          "dialog-close-btn",
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
    );

  return (
    <header className="section-header purple flex items-center justify-between gap-3 mb-3">
      <Dialog.Title>{title}</Dialog.Title>
      <Dialog.Description className="sr-only">{description}</Dialog.Description>

      {CloseButton}
    </header>
  );
}