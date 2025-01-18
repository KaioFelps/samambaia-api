import * as Dialog from "@radix-ui/react-dialog";
import { ReactNode } from "react";

export type DialogTriggerProps = {
  children: ReactNode;
};

export function DialogTrigger({ children }: DialogTriggerProps) {
  return (
    <Dialog.Trigger asChild>{children}</Dialog.Trigger>
  );
}
