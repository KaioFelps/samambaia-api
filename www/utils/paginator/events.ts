import type { PaginatorContext } from "./context";

export const events = ["next-page", "previous-page", "page-change", "page-overflow"] as const;
Object.freeze(events);

export type PaginatorEvent = (typeof events)[number];

export type PaginatorEventHandler = (context: PaginatorContext) => void;

export function getEmptyEventsMap() {
  const eventsMap = new Map();
  events.forEach((event) => {
    eventsMap.set(event, new Set());
  });
  return eventsMap;
}
