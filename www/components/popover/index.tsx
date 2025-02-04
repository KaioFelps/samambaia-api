import * as Popover from "@radix-ui/react-popover";

import { DroppableIndicator } from "../droppable-indicator";
import { PopoverContent, PopoverContentContainer } from "./content";
import { PopoverHeader } from "./header";

export default {
  Root: Popover.Root,
  Trigger: Popover.Trigger,
  Content: PopoverContent,
  ContentContainer: PopoverContentContainer,
  Header: PopoverHeader,
  Indicator: DroppableIndicator,
};
