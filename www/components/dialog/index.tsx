import * as Dialog from "@radix-ui/react-dialog";

import { DialogContent } from "./content";
import { DialogHeader } from "./header";
import { DialogTrigger } from "./trigger";

export default {
  Root: Dialog.Root,
  Content: DialogContent,
  Header: DialogHeader,
  Trigger: DialogTrigger,
  Close: Dialog.Close,
};
