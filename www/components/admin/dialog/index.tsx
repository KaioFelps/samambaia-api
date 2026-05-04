import { Close, Root, Trigger } from "@radix-ui/react-dialog";
import { DialogActionsFooter } from "./actions-footer";
import { AdminDialogContainer } from "./container";
import { AdminDialogContent } from "./content";
import { AdminDialogHeader } from "./header";

export default {
  Root,
  Trigger,
  Content: AdminDialogContent,
  Container: AdminDialogContainer,
  Header: AdminDialogHeader,
  ActionsFooter: DialogActionsFooter,
  Close,
};
