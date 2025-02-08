import { PageProps } from "@inertiajs/core/types";
import clsx from "clsx";
import { useMemo } from "react";

import { Sprite, SpriteProps } from "@/components/sprite";

type Summary = {
  users: number;
  articles: number;
  comments: number;
  team_users: number;
};

type SummaryCardProps = {
  label: string;
  count: number;
  spriteCoords: SpriteProps;
};

type AdminHomeProps = PageProps & {
  summary: Summary;
};

export default function AdminHome({ auth, summary: _summary }: AdminHomeProps) {
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

      <h1>Bem-vindo ao painel do cosmic, {auth?.user.nickname}!</h1>
    </main>
  );
}

function SummaryCard({ count, label, spriteCoords }: SummaryCardProps) {
  return (
    <article
      key={`summary-card-${label}`}
      className={clsx(
        "rounded-lg shadow-purple-700/30 shadow-lg flex gap-3 p-6 flex-1",
        "from-purple-300 to-purple-700 bg-gradient-to-br ring-inset ring-2 ring-yellow-200/30",
      )}
    >
      <Sprite
        {...spriteCoords}
        className="shadow-purple-100 [filter:_drop-shadow(4px_4px_10px_var(--tw-shadow-color))]"
      />
      <div>
        <strong className={clsx(
          "font-black text-4xl leading-none text-purple-100 text-shadow-purple-700/20",
          "[text-shadow:1px_1px_10px_var(--tw-text-shadow-color)]",
        )}
        >
          {count}
        </strong>
        <br />
        <span className="uppercase text-purple-100">{label}</span>
      </div>
    </article>
  );
}
