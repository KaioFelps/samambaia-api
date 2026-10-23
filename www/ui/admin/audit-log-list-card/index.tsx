import { CalendarBlankIcon } from "@phosphor-icons/react/dist/ssr/CalendarBlank";
import { ClockCounterClockwiseIcon } from "@phosphor-icons/react/dist/ssr/ClockCounterClockwise";
import { InfoIcon } from "@phosphor-icons/react/dist/ssr/Info";
import { NotePencilIcon } from "@phosphor-icons/react/dist/ssr/NotePencil";
import { PulseIcon } from "@phosphor-icons/react/dist/ssr/Pulse";
import { TargetIcon } from "@phosphor-icons/react/dist/ssr/Target";
import { UserIcon } from "@phosphor-icons/react/dist/ssr/User";
import { memo } from "react";

import { AdminChip } from "@/components/admin/chip";
import type { AuditLog } from "@/types/audit-log";
import { JsonModalButton } from "./json-modal-button";

const ACTION_LABELS: Record<string, string> = {
  create: "Criação (create)",
  edit: "Edição (edit)",
  delete: "Exclusão (delete)",
  access: "Acesso (access)",
};

const ACTION_VERBS: Record<string, string> = {
  create: "criou",
  edit: "editou",
  delete: "apagou",
  access: "acessou",
};

export const AuditLogListCard = memo(
  ({
    id,
    actor,
    actorReference,
    targetReference,
    target,
    actionKind,
    happenedAt,
    oldPayload,
    newPayload,
    metadata,
  }: AuditLog) => {
    const action = actionKind.toLowerCase();
    const actionLabel = ACTION_LABELS[action] ?? actionKind;
    const verb = ACTION_VERBS[action] ?? actionLabel.toLowerCase();
    const actorLabel = actor.kind === "system" ? actor.value : (actorReference ?? "—");

    const summary = [actorLabel, verb, targetReference].filter(Boolean).join(" ");

    return (
      <article className="admin-list-card">
        <span title={summary} className="flex-1/2 text-sm line-clamp-1 text-gray-800">
          {summary}
        </span>

        <div className="flex gap-2 items-center justify-end shrink-0">
          <AdminChip icon={PulseIcon} text={actionLabel} size="sm" className="whitespace-nowrap" />

          {target && (
            <AdminChip
              icon={TargetIcon}
              text={target.type}
              size="sm"
              className="whitespace-nowrap"
            />
          )}

          <AdminChip
            icon={CalendarBlankIcon}
            text={new Date(happenedAt).toLocaleString("pt-BR")}
            size="sm"
            className="whitespace-nowrap"
          />
        </div>

        <div className="flex items-center justify-end gap-1 shrink-0">
          <JsonModalButton
            title={`Ator (log #${id})`}
            icon={UserIcon}
            value={actor.kind === "entity" ? actor.value : null}
          />
          <JsonModalButton
            title={`Alvo (log #${id})`}
            icon={TargetIcon}
            value={target?.properties}
          />
          <JsonModalButton
            title={`Antes (log #${id})`}
            icon={ClockCounterClockwiseIcon}
            value={oldPayload}
          />
          <JsonModalButton title={`Depois (log #${id})`} icon={NotePencilIcon} value={newPayload} />
          <JsonModalButton title={`Metadados (log #${id})`} icon={InfoIcon} value={metadata} />
        </div>
      </article>
    );
  },
);
