export type JsonValue =
  | string
  | number
  | boolean
  | null
  | JsonValue[]
  | { [key: string]: JsonValue };

export type AuditLogActor =
  | { kind: "system"; value: string }
  | { kind: "entity"; value: JsonValue };

export type AuditLogTarget = {
  type: string;
  properties: JsonValue;
};

export type AuditLog = {
  id: number;
  actorReference: string | null;
  targetReference: string | null;
  actionKind: string;
  happenedAt: string;
  loggedAt: string;
  actor: AuditLogActor;
  target: AuditLogTarget | null;
  oldPayload: JsonValue | null;
  newPayload: JsonValue | null;
  metadata: JsonValue | null;
};
