import { Link } from "@inertiajs/react";
import clsx from "clsx";
import { useMemo } from "react";

import { FreeBadge } from "@/types/freeBadge";

export function BadgeCard({ id, image, link, linkIsExternal, code, availableUntil }: FreeBadge) {
  const isActive = useMemo(() => {
    return !availableUntil || new Date(availableUntil) > new Date();
  }
  , [availableUntil]);

  const Anchor = useMemo(() => linkIsExternal
    ? "a"
    : Link, [linkIsExternal,

  ]);

  return (
    <Anchor
      key={"free-badge-link-" + id}
      href={link}
      target={linkIsExternal
        ? "_blank"
        : "_self"}
      rel={linkIsExternal
        ? "noreferrer"
        : undefined}
      data-active={isActive}
      tabIndex={!isActive
        ? -1
        : undefined}
      className={clsx(
        "aspect-square flex-1 grid place-items-center rounded-lg bg-gray-250",
        "border-2 border-black transition-all duration-75",
        "outline-none ring-green-500/50 ring-0 data-[active=true]:focus:ring-4",
        "data-[active=false]:saturate-0 data-[active=false]:cursor-not-allowed",
        "data-[active=false]:opacity-50 data-[active=false]:pointer-events-none",
        "data-[active=true]:hover:bg-green-500 data-[active=true]:active:bg-green-600",
        "shadow-black/15 from-white data-[active=true]:hover:from-white/50",
        // eslint-disable-next-line @stylistic/max-len
        "[box-shadow:_inset_0_-3px_0_0_var(--tw-shadow-color),_inset_0_3px_0_0_var(--tw-gradient-from),_var(--tw-ring-offset-shadow),_var(--tw-ring-shadow)]",
      )}
    >
      <img
        src={image}
        alt={`Emblema grátis de código ${code}`}
        className="relative"
      />
    </Anchor>
  );
}
