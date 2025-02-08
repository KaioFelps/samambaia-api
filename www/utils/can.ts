import { TPermission } from "@/types/auth";

export type CanMode = "some" | "every";

export function can(
  userPermissions: TPermission[],
  required: TPermission[],
  mode: CanMode = "some",
): boolean {
  switch (mode) {
    case "every":
      return required.every(requiredPermission => userPermissions.includes(requiredPermission));
    case "some":
      return required.some(requiredPermission => userPermissions.includes(requiredPermission));
  }
}
