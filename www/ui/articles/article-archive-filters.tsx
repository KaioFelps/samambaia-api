import * as RadioGroup from "@radix-ui/react-radio-group";
import clsx from "clsx";
import type { FormEvent } from "react";
import PublicButton from "@/components/button/public-button";
import { Chip } from "@/components/chip";
import PublicForm from "@/components/form/public-form";
import { Sprite } from "@/components/sprite";
import type { ArchiveFilters, TextFilter } from "@/pages/articles/hooks";
import type { ArticleTag } from "@/types/article-tag";

const TEXT_FILTERS: { label: string; value: TextFilter }[] = [
  { label: "Título", value: "title" },
  { label: "Autor", value: "author" },
];

type ArticleArchiveFiltersProps = {
  tags: ArticleTag[];
  archiveFilters: ArchiveFilters;
};

export function ArticleArchiveFilters({ tags, archiveFilters }: ArticleArchiveFiltersProps) {
  const {
    filter,
    setFilter,
    query,
    setQuery,
    processing,
    selectedTagIds,
    submitTextFilter,
    toggleTag,
  } = archiveFilters;

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    submitTextFilter();
  }

  return (
    <>
      <search>
        <form className="flex items-center gap-2 mb-2" onSubmit={handleSubmit}>
          <PublicForm.Input
            name="public-archive-search"
            label="Busca"
            hiddenLabel
            type="text"
            placeholder={`Buscar por ${filter === "title" ? "título" : "autor"}`}
            className="text-input ring-blue-500/40 focus-within:border-blue-700"
            containerClassName="w-full"
            value={query}
            onInput={(event) => setQuery(event.currentTarget.value)}
          />
          <PublicButton.Default variant="blue" size="lg" type="submit" disabled={processing}>
            <Sprite width={17} height={16} x={0} y={-96} />
            {processing ? "Pesquisando..." : "Pesquisar"}
          </PublicButton.Default>
        </form>
      </search>

      <div className="flex gap-2 divide-x divide-black/20">
        <RadioGroup.Root
          aria-label="Buscar por"
          value={filter}
          onValueChange={(value: TextFilter) => setFilter(value)}
          disabled={processing}
          className="flex gap-1 pr-2">
          {TEXT_FILTERS.map(({ label, value }) => (
            <RadioGroup.Item key={`article-archive-text-filter-${value}`} value={value} asChild>
              <Chip asChild>
                <button
                  type="button"
                  className={clsx(
                    "self-start data-[state=checked]:cursor-default data-[state=checked]:bg-gray-500",
                    "data-[state=checked]:border-gray-600 data-[state=checked]:ring-gray-600/40",
                  )}>
                  {label}
                </button>
              </Chip>
            </RadioGroup.Item>
          ))}
        </RadioGroup.Root>

        {tags.length > 0 && (
          <div
            id="article-archive-filter-tags-container"
            className="flex flex-wrap items-row gap-1">
            {tags.map((tag) => {
              const active = selectedTagIds.includes(tag.id.toString());

              return (
                <Chip asChild key={`article-archive-filter-by-tag-${tag.id}`}>
                  <button
                    type="button"
                    aria-pressed={active}
                    data-active={active}
                    disabled={processing}
                    onClick={() => toggleTag(tag.id)}>
                    Tag: {tag.value}
                  </button>
                </Chip>
              );
            })}
          </div>
        )}
      </div>
    </>
  );
}
