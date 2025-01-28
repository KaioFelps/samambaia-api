import * as Dialog from "@radix-ui/react-dialog";
import { memo, ReactNode } from "react";

export type DialogTriggerProps = {
  children: ReactNode;
};

export const DialogTrigger = memo(({ children }: DialogTriggerProps) => {
  return (
    <Dialog.Trigger asChild>{children}</Dialog.Trigger>
  );
});
