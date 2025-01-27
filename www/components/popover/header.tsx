import { Close } from "@radix-ui/react-popover";

import { Sprite } from "../sprite";

type PopoverHeaderProps = {
  title: string;
  omitClose?: boolean;
};

export function PopoverHeader({ title, omitClose = false }: PopoverHeaderProps) {
  return (
    <header className="
        flex items-center justify-between px-3 py-1 mb-2 font-bold text-white
        border-b-2 border-black shadow-white/10 shadow-[0_2px_0_0]
        "
    >
      <span>{title}</span>

      {!omitClose && (
        <Close
          title="Fechar popover"
          aria-describedby="Fechar a janela pop-over"
          autoFocus={false}
          className="dialog-close-btn"
        >
          <Sprite
            x={-160}
            y={-63}
            width={20}
            height={20}
          />
        </Close>
      )}
    </header>
  );
}
