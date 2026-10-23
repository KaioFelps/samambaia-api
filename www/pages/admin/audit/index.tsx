import type { PageProps } from "@inertiajs/core/types";
import { router } from "@inertiajs/react";
import { useState } from "react";

import { Alert } from "@/components/alert";
import { Head } from "@/components/head";
import Header from "@/components/header";
import { Main } from "@/components/main";
import Pagination from "@/components/pagination";
import { useMemoizedPaginatorParameters } from "@/hooks/pagination";
import type { AuditLog } from "@/types/audit-log";
import type { Paginated } from "@/types/pagination";
import { AuditLogListCard } from "@/ui/admin/audit-log-list-card";
import TableHeader from "@/ui/admin/paginated-table-header";

type AdminAuditHomeProps = PageProps & {
  logs: Paginated<AuditLog[]>;
};

export default function AdminAuditHome({ logs }: AdminAuditHomeProps) {
  const paginatorArgs = useMemoizedPaginatorParameters({
    lastPage: logs.pagination.totalPages,
    visibleButtons: 7,
    currentPage: logs.pagination.currentPage,
    align: "left",
  });

  const [filterIsLoading, setFilterIsLoading] = useState(false);

  function handleFilter({ filter, query }: { filter?: string; query?: string }) {
    const destination = !filter || !query ? "?" : `?${filter}=${encodeURIComponent(query)}`;

    router.visit(destination, {
      only: ["logs"],
      onStart: () => setFilterIsLoading(true),
      onFinish: () => setFilterIsLoading(false),
    });
  }

  return (
    <>
      <Head admin title="Logs de auditoria" />

      <Main admin>
        <Header.Root className="mb-8">
          <Header.Title>Logs de auditoria</Header.Title>
          <Header.Divisor />
        </Header.Root>

        <TableHeader.Root className="mb-4">
          <TableHeader.Count
            currentPage={logs.pagination.currentPage}
            itemsInCurrentPage={logs.data.length}
            itemsPerPage={logs.pagination.itemsPerPage}
            totalItems={logs.pagination.totalItems}
            subject="logs"
          />
          <TableHeader.Filter
            filters={[{ label: "Busca", value: "search" }]}
            handleFilter={handleFilter}
            loading={filterIsLoading}
          />
        </TableHeader.Root>

        {logs.pagination.totalItems > 0 ? (
          <div className="flex flex-col gap-1">
            {logs.data.map((log) => (
              <AuditLogListCard key={`admin-audit-log-list-${log.id}`} {...log} />
            ))}
          </div>
        ) : (
          <Alert admin type="info" message="Nenhum log de auditoria encontrado." />
        )}

        {paginatorArgs && (
          <Pagination.Root paginatorArgs={paginatorArgs}>
            <Pagination.Container className="mt-8 justify-end">
              <Pagination.ArrowButton admin direction="backward" />
              <Pagination.Buttons admin />
              <Pagination.ArrowButton admin direction="forward" />
            </Pagination.Container>
          </Pagination.Root>
        )}
      </Main>
    </>
  );
}
