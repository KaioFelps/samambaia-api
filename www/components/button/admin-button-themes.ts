import clsx from "clsx";

const adminButtonStyles = {
  default: {
    default: clsx(
      "bg-blue-500 border border-black/10 text-white",
      "ring-blue-500/40 enabled:hover:bg-blue-600 enabled:active:brightness-90",
      "disabled:bg-gray-400"),
    success: clsx(
      "bg-green-500 border border-black/10 text-white",
      "ring-green-700/40 enabled:hover:bg-green-600 enabled:active:bg-green-700",
      "disabled:bg-gray-400"),
    danger: clsx(
      "bg-red-700 border-black/10 text-white",
      "ring-red-700/40 enabled:hover:bg-red-800 enabled:active:brightness-90",
      "disabled:bg-gray-400"),
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
