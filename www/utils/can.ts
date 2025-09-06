import type { TPermission } from "@/types/auth";

export type CanMode = "some" | "every";

export function can(
  userPermissions: TPermission[] | undefined,
  required: TPermission | TPermission[],
  mode: CanMode = "some",
): boolean {
  if (!userPermissions) return false;

  const requiredPerms = Array.isArray(required) ? required : [required];

  switch (mode) {
    case "every":
      return requiredPerms.every((requiredPerm) => userPermissions.includes(requiredPerm));
    case "some":
      return requiredPerms.some((requiredPerm) => userPermissions.includes(requiredPerm));
  }
}
