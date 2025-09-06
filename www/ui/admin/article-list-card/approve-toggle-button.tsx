import { CheckFat } from "@phosphor-icons/react/dist/ssr/CheckFat";
import { Spinner } from "@phosphor-icons/react/dist/ssr/Spinner";
import clsx from "clsx";
import { memo } from "react";

type PublishmentCheckProps = {
  isPublished: boolean;
  isLoading: boolean;
  onClick: () => void;
};

export const ApproveToggleButton = memo(
  ({ isPublished, isLoading, onClick }: PublishmentCheckProps) => {
    return (
      <button
        type="button"
        disabled={isLoading}
        onClick={onClick}
        className={clsx(
          "p-1 rounded-full transition-all self-center outline-hidden ring-0 focus-visible:ring-4",
          !isLoading && isPublished
            ? "text-white bg-green-500 hover:bg-green-600 active:bg-green-700 ring-green-600/40"
            : "text-gray-700 bg-gray-300 hover:bg-gray-400 active:brightness-95 ring-purple-500/40",
        )}>
        {isLoading ? (
          <Spinner className="animate-spin" size={16} weight="bold" />
        ) : (
          <CheckFat size={16} weight="fill" />
        )}
      </button>
    );
  },
);
