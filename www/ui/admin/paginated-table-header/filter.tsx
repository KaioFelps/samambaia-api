import { usePage } from "@inertiajs/react";
import { MagnifyingGlass } from "@phosphor-icons/react/dist/ssr/MagnifyingGlass";
import clsx from "clsx";
import { type FormEvent, memo, useCallback, useEffect, useState } from "react";

import Button from "@/components/button";
import { AdminDroppableArrow } from "@/components/droppable-arrow";
import { AdminDroppableIndicator } from "@/components/droppable-indicator";
import Select from "@/components/select";
import { PaginationPolitics } from "@/core/politics/pagination-politics";

type PaginatedTableHeaderFilterProps = {
  filters: { label: string; value: string }[];
  selectedFilter?: string;
  loading?: boolean;
  handleFilter: (_args: { filter?: string; query?: string }) => void;
};

export const PaginatedTableHeaderFilter = memo(
  ({ filters, handleFilter, loading }: PaginatedTableHeaderFilterProps) => {
    const [filter, setFilter] = useState<string | undefined>();
    const [query, setQuery] = useState<string | undefined>();

    const pageUrl = usePage().url;

    const handleSubmitFilter = useCallback(
      (event: FormEvent) => {
        event.preventDefault();
        handleFilter({ filter, query });
      },
      [filter, query, handleFilter],
    );

    useEffect(() => {
      const search = Object.entries(PaginationPolitics.getQueryObjectFromUrl(pageUrl)).find(
        ([key]) => filters.some((filter) => filter.value === key),
      );

      if (search) {
        setFilter(search[0]);
        setQuery(search[1]);
      }
    }, [filters, pageUrl]);

    return (
      <search>
        <form className="flex items-center gap-2" onSubmit={handleSubmitFilter}>
          <label
            className={clsx(
              "flex items-center gap-1.5 self-stretch px-3 rounded-lg border border-black/20",
              "transition-all duration-100",
              "ring-0 ring-purple-500/40 has-focus:ring-4",
            )}>
            <MagnifyingGlass size={16} weight="bold" />
            <input
              type="text"
              className="leading-none text-sm font-light p-0 outline-hidden"
              placeholder="Busca"
              defaultValue={query}
              onInput={(event) => {
                setQuery((event.target as HTMLInputElement).value);
              }}
            />
          </label>

          <Select.Root value={filter} onValueChange={setFilter}>
            <Select.Trigger asChild>
              <Button admin className="group" variant="ghost">
                <Select.Value placeholder="Filtro" />
                <AdminDroppableIndicator />
              </Button>
            </Select.Trigger>

            <Select.Content>
              <Select.Viewport>
                {filters.map(({ label, value }) => (
                  <Select.Item
                    key={`table-header-select-${value}-option`}
                    label={label}
                    value={value}
                  />
                ))}
              </Select.Viewport>
              <AdminDroppableArrow component="select" />
            </Select.Content>
          </Select.Root>

          <Button admin type="submit" disabled={loading} className="justify-self-end">
            {loading ? "Pesquisando..." : "Pesquisar"}
          </Button>
        </form>
      </search>
    );
  },
);
