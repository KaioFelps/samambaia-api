import clsx from "clsx";

export function AnnouncementsSliderSkeleton() {
  return (
    <div
      className={clsx(
        "w-full bg-purple-700 aspect-square rounded-lg border-2 border-black",
        "shadow-black/25 shadow-[0_2px_0_0] animate-pulse",
      )}
    />
  );
}
