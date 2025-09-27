import type { ButtonHTMLAttributes } from "react";
import { DefaultPublicButton } from "./default";
import { GhostPublicButton } from "./ghost";

export type CorePublicButtonProps = {
  asChild?: boolean;
  size?: "md" | "lg";
} & ButtonHTMLAttributes<HTMLButtonElement>;

export default {
  Default: DefaultPublicButton,
  Ghost: GhostPublicButton,
};
