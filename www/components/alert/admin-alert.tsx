import { Info } from "@phosphor-icons/react/dist/ssr";
import clsx from "clsx";
import { memo, useMemo } from "react";

import { BaseAlertProps } from ".";

export type AdminAlertProps = BaseAlertProps & {
  type: "info";
};

export const AdminAlert = memo(({ message, type, className }: AdminAlertProps) => {
  const Icon = useMemo(() => {
    switch (type) {
      case "info": return Info;
    }
  }, [type]);

  return (
    <div className={clsx(
      "rounded-ss-lg rounded-se-lg border-b flex items-center gap-1.5 p-2.5",
      "text-sm font-normal text-gray-800",

      // info-type
      type === "info" && "border-blue-500 bg-blue-500/10",

      className && className,
    )}
    >
      <Icon
        size={24}
        weight="fill"
        className={clsx(
          type === "info" && "text-blue-700",
        )}
      />
      {message}
    </div>
  );
});
