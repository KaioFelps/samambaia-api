import { CheckFat } from "@phosphor-icons/react/dist/ssr/CheckFat";
import { Info } from "@phosphor-icons/react/dist/ssr/Info";
import { SealWarning } from "@phosphor-icons/react/dist/ssr/SealWarning";
import clsx from "clsx";
import { memo, useMemo } from "react";

import type { BaseAlertProps } from ".";

export type AdminAlertProps = BaseAlertProps & {
  type: "info" | "warning" | "success";
};

export const AdminAlert = memo(({ message, type, className }: AdminAlertProps) => {
  const Icon = useMemo(() => {
    switch (type) {
      case "info":
        return Info;
      case "warning":
        return SealWarning;
      case "success":
        return CheckFat;
    }
  }, [type]);

  return (
    <div
      className={clsx(
        "rounded-ss-lg rounded-se-lg border-b flex items-center gap-1.5 p-2.5",
        "text-sm font-normal text-gray-800",

        // info-type
        type === "info" && "border-blue-500 bg-blue-500/10",
        type === "warning" && "border-yellow-700 bg-yellow-500/25",
        type === "success" && "border-green-700 bg-green-500/25",

        className && className,
      )}>
      <Icon
        size={24}
        weight="fill"
        className={clsx(
          type === "info" && "text-blue-700",
          type === "warning" && "text-yellow-900",
          type === "success" && "text-green-700",
        )}
      />
      {message}
    </div>
  );
});
