import clsx from "clsx";

type SidebarSectionTitleProps = {
  title: string;
};

export const SidebarSectionTitle = ({ title }: SidebarSectionTitleProps) => {
  return (
    <label className={clsx(
      "block w-full px-3 pt-3 pb-1 text-xs text-gray-700 border-t border-black/10",
    )}
    >
      {title}
    </label>
  );
};
