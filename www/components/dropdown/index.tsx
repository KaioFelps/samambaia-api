import * as DropdownMenu from "@radix-ui/react-dropdown-menu";

import { DroppableIndicator } from "../droppable-indicator";
import { DropdownContent } from "./content";
import { DropdownTrigger } from "./trigger";

export default {
  Root: DropdownMenu.Root,
  Trigger: DropdownTrigger,
  Content: DropdownContent,
  Indicator: DroppableIndicator,
};
