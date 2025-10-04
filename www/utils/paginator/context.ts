import type { Paginator } from ".";

export class PaginatorContext {
  public constructor(
    public readonly currentPage: number,
    public readonly lastPage: number,
    public readonly hadOverflow: boolean,
    public readonly paginator: Paginator,
    public readonly previousCurrentPage?: number,
  ) {}
}
