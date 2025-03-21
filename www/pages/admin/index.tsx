import { PageProps } from "@inertiajs/core/types";
import { useMemo } from "react";

import { Alert } from "@/components/alert";
import Button from "@/components/button";
import Header from "@/components/header";
import { CouncilAlertCard } from "@/ui/admin/council-alert-card";
import { SummaryCard, SummaryCardProps } from "@/ui/admin/summary-card";

import { CouncilAlert } from "./www/types/council-alert";

type Summary = {
  users: number;
  articles: number;
  comments: number;
  team_users: number;
};

type AdminHomeProps = PageProps & {
  summary: Summary;
  councilAlerts: CouncilAlert[];
};

export default function AdminHome({ summary: _summary, councilAlerts }: AdminHomeProps) {
  const summary: SummaryCardProps[] = useMemo(() => [
    {
      label: "notícias",
      count: _summary.articles,
      spriteCoords: { x: 0, y: 0, width: 40, height: 40 },
    },
    {
      label: "comentários",
      count: _summary.comments,
      spriteCoords: { x: -459, y: -2, width: 40, height: 40 },
    },
    {
      label: "usuários",
      count: _summary.users,
      spriteCoords: { x: -504, y: -44, width: 40, height: 40 },
    },
    {
      label: "membros da equipe",
      count: _summary.team_users,
      spriteCoords: { x: -506, y: 0, width: 40, height: 40 },
    },
  ] as SummaryCardProps[], [_summary]);

  return (
    <main className="admin-main-container">
      <section
        id="summary"
        className="flex gap-3 p-6 bg-white/40 rounded-lg -mx-3 mb-12"
      >
        {summary.map(SummaryCard)}
      </section>

      <section>
        <Header.Root className="mb-6">
          <Header.Title heading="h2">Avisos do Grêmio</Header.Title>
          <Header.Divisor />
          <Header.Actions>
            <Button
              admin
              asLink
              href=""
            >
              Adicionar novo aviso
            </Button>
          </Header.Actions>
        </Header.Root>

        <div className="flex flex-col gap-6">
          {councilAlerts.length
            ? councilAlerts.map((alert) => (
              <CouncilAlertCard
                key={`council-alert-card-${alert.id}`}
                {...alert}
              />))
            : (
              <Alert
                admin
                type="info"
                message="Não há nenhum aviso do grêmio."
              />
              )}
        </div>
      </section>
    </main>
  );
}
