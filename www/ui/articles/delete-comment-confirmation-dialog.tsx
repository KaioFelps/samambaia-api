import { forwardRef, type ReactElement } from "react";
import PublicButton from "@/components/button/public-button";
import Dialog from "@/components/dialog";

type Props = {
  trigger: ReactElement;
  buttonsShallBeDisabled: boolean;
  onConfirm: () => void;
};

export const DeleteCommentConfirmationDialog = forwardRef<HTMLButtonElement, Props>(
  ({ trigger, onConfirm, buttonsShallBeDisabled, ...props }, ref) => {
    return (
      <Dialog.Root>
        <Dialog.Trigger asChild {...props} ref={ref}>
          {trigger}
        </Dialog.Trigger>
        <Dialog.Content>
          <Dialog.Header title="Apagar comentário" />
          <p className="mb-3">
            Tem certeza de que deseja apagar o seu comentário? Essa ação é irreversível.
          </p>
          <div className="flex items-center gap-2 justify-end">
            <Dialog.Close asChild>
              <PublicButton.Default
                disabled={buttonsShallBeDisabled}
                aria-busy={buttonsShallBeDisabled}
                type="button">
                Cancelar
              </PublicButton.Default>
            </Dialog.Close>

            <Dialog.Close asChild>
              <PublicButton.Default
                disabled={buttonsShallBeDisabled}
                aria-busy={buttonsShallBeDisabled}
                onClick={onConfirm}
                type="button"
                variant="yellow">
                Confirmar
              </PublicButton.Default>
            </Dialog.Close>
          </div>
        </Dialog.Content>
      </Dialog.Root>
    );
  },
);
