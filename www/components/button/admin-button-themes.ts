import clsx from "clsx";

const adminButtonStyles = {
  default: {
    default: clsx(
      "bg-blue-500 border border-black/10 text-white",
      "ring-blue-500/40 hover:bg-blue-600 active:brightness-90"),
    success: clsx(
      "bg-green-500 border border-black/10 text-white",
      "ring-green-700/40 hover:bg-green-600 active:bg-green-700"),
  },
  ghost: {
    default: clsx(
      "border border-black/20 ring-purple-500/40 bg-transparent",
      "hover:bg-black/5 active:bg-black/10"),
  },
} as const;

type AdminButtonStyles = typeof adminButtonStyles;

type VariantAndTheme<T extends keyof AdminButtonStyles = keyof AdminButtonStyles> =
 T extends keyof AdminButtonStyles ?
     { variant: T; theme: keyof AdminButtonStyles[T] }
   : never;

export type OptionalVariantAndTheme = {
  variant?: never;
  theme?: never;
}
| {
  variant: keyof AdminButtonStyles;
  theme?: never;
}
| VariantAndTheme;

export type ThemeKey = keyof AdminButtonStyles[keyof AdminButtonStyles];

export function getAdminButtonStyles({ variant, theme }: VariantAndTheme) {
  return adminButtonStyles[variant][theme as ThemeKey];
}

Object.freeze(adminButtonStyles);
