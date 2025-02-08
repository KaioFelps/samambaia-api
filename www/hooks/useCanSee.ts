import { usePage } from "@inertiajs/react";

import { TPermission } from "@/types/auth";
import { can, CanMode } from "@/utils/can";

export function useCanSee(
  requires: TPermission[] | TPermission | undefined,
  mode?: CanMode,
): boolean {
  const userPermissions = usePage().props.auth?.permissions;

  if (!requires) return true;

  const requiredPermissions = Array.isArray(requires)
    ? requires
    : [requires];

  if (requiredPermissions.length === 0 || !userPermissions) return false;

  return can(userPermissions, requiredPermissions, mode);
}
