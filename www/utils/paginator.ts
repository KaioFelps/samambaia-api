import { GetVisibleButtonsParams, PaginationPolitics } from "@/core/politics/pagination-politics";
import { IllegalArgumentException } from "@/exceptions/illegal-argument-exception";

export type SearchParameters = Record<string, string | number | undefined>;

const events = ["next-page", "previous-page", "page-change"] as const;
Object.freeze(events);

type PaginatorEvent = typeof events[number];
type PaginationLink = { page: number; link: string };
export type GetPaginationParams = {
  queryString?: string;
  extraArgs?: SearchParameters;
};
export type PaginatorConstructorArgs = {
  url: string;
  currentPage?: number;
  lastPage: number;
  visibleButtons?: number;
  align?: GetVisibleButtonsParams["align"];
  pageQuery?: string;
};

export class Paginator {
  private url!: string;
  private currentPage!: number;
  private lastPage!: number;
  private visibleButtons!: number;
  private align!: GetVisibleButtonsParams["align"];
  private pageQuery!: string;

  private eventListeners: Map<PaginatorEvent, Set<(_page: number) => void>> = new Map();

  public constructor({
    currentPage,
    lastPage,
    url,
    align,
    pageQuery,
    visibleButtons,
  }: PaginatorConstructorArgs) {
    this.url = url;
    this.setLastPage(lastPage);

    this.align = align ?? "center";
    this.setPageQuery(pageQuery ?? "page");
    this.setCurrentPage(currentPage ?? 1);
    this.setVisibleButtons(visibleButtons ?? PaginationPolitics.DEFAULT_VISIBLE_BUTTONS);

    events.forEach(event => {
      this.eventListeners.set(event, new Set());
    });
  }

  public setPageQuery(pageQuery: string) {
    if (pageQuery.includes(" ") || pageQuery.includes("\n")) {
      throw new IllegalArgumentException(
        "Paginator's page key cannot contain whitespaces nor EOLs.");
    }

    if (pageQuery.length < 1) {
      throw new IllegalArgumentException("Paginator's page key must be at least 1 character long.");
    }

    this.pageQuery = pageQuery;
  }

  public setVisibleButtons(quantity: number) {
    if (quantity <= 0) {
      throw new IllegalArgumentException(
        "Paginator needs to be able to display at least one page.");
    }

    if (quantity > PaginationPolitics.MAX_VISIBLE_BUTTONS) {
      throw new IllegalArgumentException(
        "Paginator cannot display more than " +
        PaginationPolitics.MAX_VISIBLE_BUTTONS +
        " pages.");
    }

    const isOddQuantity = quantity % 2 !== 0;
    if (!isOddQuantity && this.align === "center") {
      throw new IllegalArgumentException(
        "Tried to add a even amount of " + quantity + " visible buttons, but align is " +
          this.align + ", which only accepts odd amount of visible buttons.");
    }

    this.visibleButtons = quantity;
  }

  public setLastPage(lastPage: number) {
    if (lastPage < this.currentPage) {
      throw new IllegalArgumentException(
        "Last page cannot be lower than current page.");
    }

    this.lastPage = lastPage;
  }

  public setCurrentPage(page: number) {
    if (page > this.lastPage) {
      throw new IllegalArgumentException(
        "Paginator's current page " + page + " can't be beyond the last page " +
        this.lastPage + ".");
    }

    if (page <= 0) {
      throw new IllegalArgumentException(
        "Tried to assign " + page + " to Paginator's current page," +
        "but it can't be lower than 1.");
    }

    this.currentPage = page;
    this.callEventListeners("page-change");
  }

  public hasPreviousPage(): boolean {
    return this.currentPage > 1;
  }

  public hasNextPage(): boolean {
    return this.currentPage < this.lastPage;
  }

  public previousPage() {
    if (this.currentPage - 1 < 1) {
      console.warn(
        "Tried to go to an previous page, but there is none. Current page is " +
        this.currentPage + ".",
      );

      return;
    }

    this.currentPage--;
    this.callEventListeners("previous-page");
  }

  public nextPage() {
    if (this.currentPage + 1 > this.lastPage) {
      console.warn(
        "Tried to go to the next page, but it doesn't exist. Current page is " +
            this.currentPage + " and last page is " + this.lastPage + ".",
      );

      return;
    }

    this.currentPage++;
    this.callEventListeners("next-page");
  }

  public getCurrentPage() {
    return this.currentPage;
  }

  public getCurrentPagePagination(params: GetPaginationParams = {}): PaginationLink {
    return this.getPaginationLinkForPage(this.currentPage, params);
  }

  public getPagination({
    queryString,
    extraArgs,
  }: GetPaginationParams = {}): Array<PaginationLink> {
    const boundaries = PaginationPolitics.getVisibleButtons({
      align: this.align,
      lastPage: this.lastPage,
      currentPage: this.currentPage,
      visibleButtons: this.visibleButtons,
    });

    const pages = [];
    for (let i = boundaries.maxLeft; i <= boundaries.maxRight; i++) pages.push(i);

    queryString = this.getPaginationQueryString(queryString, extraArgs);

    return pages.map(page => ({
      page,
      link: this.preparePaginationLink(queryString, page),
    }));
  }

  public addEventListener(event: PaginatorEvent, callback: () => void) {
    this.eventListeners.get(event)?.add(callback);
  }

  public removeEventListener(event: PaginatorEvent, callback: () => void) {
    this.eventListeners.get(event)?.delete(callback);
  }

  private callEventListeners(event: PaginatorEvent) {
    this.eventListeners.get(event)?.forEach(callback => callback(this.currentPage));
  }

  public countEventListeners(event: PaginatorEvent): number {
    return this.eventListeners.get(event)?.size ?? 0;
  }

  public resetEventListeners() {
    events.forEach(event => this.eventListeners.set(event, new Set()));
  }

  public getPaginationLinkForPage(
    page: number,
    {
      extraArgs,
      queryString,
    }: GetPaginationParams = {},
  ): PaginationLink {
    queryString = this.getPaginationQueryString(queryString, extraArgs);

    return {
      page: this.currentPage,
      link: this.preparePaginationLink(queryString, page),
    };
  }

  private getPaginationQueryString(
    queryString: string | undefined,
    extraArgs: SearchParameters | undefined,
  ) {
    let searchParameters: SearchParameters = {};

    if (queryString) {
      searchParameters = this.getQueryObjectFromUrl(queryString, searchParameters);
    }

    if (extraArgs) searchParameters = { ...searchParameters, ...extraArgs };

    if (this.pageQuery in searchParameters) delete searchParameters[this.pageQuery];

    return this.getQueryStringFromArgs(searchParameters);
  }

  private getQueryObjectFromUrl(url: string, args: SearchParameters = {}) {
    let queryString = url;

    if (queryString.indexOf("?") === -1) {
      return {};
    }

    queryString = queryString.substring(queryString.indexOf("?") + 1);

    return Object.assign(Object.fromEntries(new URLSearchParams(queryString)), args);
  }

  private getQueryStringFromArgs(args: SearchParameters = {}) {
    const params = Object.entries(args).map(([key, value]) => {
      if (value) return `${key}=${value}`;
      return key;
    });

    const queryString = "?" + params.join("&");

    return queryString;
  }

  private preparePaginationLink(queryString: string, page: number): string {
    if (queryString === "?") return this.url + "?" + this.pageQuery + "=" + page;
    return this.url + queryString + "&" + this.pageQuery + "=" + page;
  }
}
