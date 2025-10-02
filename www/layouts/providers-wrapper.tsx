import { TooltipProvider } from "@radix-ui/react-tooltip";
import type { PropsWithChildren } from "react";

export function ProvidersWrapper({ children }: PropsWithChildren) {
  return <TooltipProvider delayDuration={100}>{children}</TooltipProvider>;
}
