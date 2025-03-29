import { Close, Root, Trigger } from "@radix-ui/react-dialog";

import { AdminDialogContainer } from "./container";
import { AdminDialogContent } from "./content";
import { AdminDialogHeader } from "./header";

export default {
  Root,
  Trigger,
  Content: AdminDialogContent,
  Container: AdminDialogContainer,
  Header: AdminDialogHeader,
  Close,
};
