import {
  Root as PRoot,
  Trigger as PTrigger,
  Value as PValue,
} from "@radix-ui/react-select";

import { SelectContent } from "./content";
import { SelectItem } from "./item";
import { SelectViewport } from "./view-port";

export default {
  Root: PRoot,
  Trigger: PTrigger,
  Value: PValue,
  Content: SelectContent,
  Item: SelectItem,
  Viewport: SelectViewport,
};
