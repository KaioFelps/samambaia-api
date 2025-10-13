import { type ButtonHTMLAttributes, forwardRef, type ReactElement } from "react";
import PublicButton from "@/components/button/public-button";
import Dialog from "@/components/dialog";

type Props = {
  trigger: ReactElement;
} & ButtonHTMLAttributes<HTMLButtonElement>;

export const ReportNeedsLoginDialog = forwardRef<HTMLButtonElement, Props>(
  ({ trigger, ...props }, ref) => {
    return (
      <Dialog.Root>
        <Dialog.Trigger asChild {...props} ref={ref}>
          {trigger}
        </Dialog.Trigger>
        <Dialog.Content>
          <Dialog.Header title="Denunciar comentário" />
          <p className="mb-3">Você precisa estar logado para poder denunciar um comentário.</p>
          <div className="flex items-center gap-2 justify-end">
            <Dialog.Close asChild>
              <PublicButton.Default type="button" variant="default">
                Entendido
              </PublicButton.Default>
            </Dialog.Close>
          </div>
        </Dialog.Content>
      </Dialog.Root>
    );
  },
);
