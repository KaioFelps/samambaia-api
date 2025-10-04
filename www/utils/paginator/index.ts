import {
  type GetVisibleButtonsParams,
  PaginationPolitics,
  type SearchRecord,
} from "@/core/politics/pagination-politics";
import { IllegalArgumentException } from "@/exceptions/illegal-argument-exception";
import { PaginatorContext } from "./context";
import {
  events,
  getEmptyEventsMap,
  type PaginatorEvent,
  type PaginatorEventHandler,
} from "./events";
import type { PaginatorFlags } from "./flags";

type PaginationLink = { page: number; link: string };

export type GetPaginationParams = {
  queryString?: string;
  extraArgs?: SearchRecord;
};
export type PaginatorConstructorArgs = {
  url: string;
  currentPage?: number;
  lastPage?: number;
  visibleButtons?: number;
  align?: GetVisibleButtonsParams["align"];
  pageQuery?: string;
  eventListeners?: Map<PaginatorEvent, Set<PaginatorEventHandler>>;
  flags?: PaginatorFlags;
};

export class Paginator {
  private url: string = "";
  private currentPage: number = 1;
  private lastPage: number = 1;
  private visibleButtons: number = PaginationPolitics.DEFAULT_VISIBLE_BUTTONS;
  private align: GetVisibleButtonsParams["align"] = "center";
  private pageQuery: string = "page";
  private flags: PaginatorFlags = {
    ignoreErrorOnOverflow: false,
    ignoreErrorOnUnderflow: false,
  };

  private eventListeners: Map<PaginatorEvent, Set<PaginatorEventHandler>> = getEmptyEventsMap();

  public constructor(args?: PaginatorConstructorArgs) {
    if (args) {
      const {
        lastPage,
        url,
        align,
        currentPage,
        pageQuery,
        visibleButtons,
        eventListeners,
        flags,
      } = args;
      this.url = url;

      if (lastPage !== undefined) this.setLastPage(lastPage);
      if (align !== undefined) this.align = align;
      if (pageQuery !== undefined) this.setPageQuery(pageQuery);
      if (currentPage !== undefined) this.setCurrentPage(currentPage);
      if (visibleButtons !== undefined) this.setVisibleButtons(visibleButtons);
      if (eventListeners !== undefined) this.eventListeners = eventListeners;
      if (flags) this.flags = flags;
    }
  }

  public setPageQuery(pageQuery: string) {
    if (pageQuery.includes(" ") || pageQuery.includes("\n")) {
      throw new IllegalArgumentException(
        "Paginator's page key cannot contain whitespaces nor EOLs.",
      );
    }

    if (pageQuery.length < 1) {
      throw new IllegalArgumentException("Paginator's page key must be at least 1 character long.");
    }

    this.pageQuery = pageQuery;
  }

  public setVisibleButtons(quantity: number) {
    if (quantity <= 0) {
      throw new IllegalArgumentException(
        "Paginator needs to be able to display at least one page.",
      );
    }

    if (quantity > PaginationPolitics.MAX_VISIBLE_BUTTONS) {
      throw new IllegalArgumentException(
        `Paginator cannot display more than ${PaginationPolitics.MAX_VISIBLE_BUTTONS} pages.`,
      );
    }

    const isOddQuantity = quantity % 2 !== 0;
    if (!isOddQuantity && this.align === "center") {
      throw new IllegalArgumentException(
        "Tried to add a even amount of " +
          quantity +
          " visible buttons, but align is " +
          this.align +
          ", which only accepts odd amount of visible buttons.",
      );
    }

    this.visibleButtons = quantity;
  }

  public setLastPage(lastPage: number) {
    if (lastPage < this.currentPage && !this.flags.ignoreErrorOnUnderflow) {
      throw new IllegalArgumentException("Last page cannot be lower than current page.");
    }

    this.lastPage = lastPage;
  }

  public setCurrentPage(page: number) {
    if (page > this.lastPage) {
      if (this.countEventListeners("page-overflow") > 0) {
        this.callEventListeners("page-overflow", undefined, true);
        return;
      }

      if (this.flags.ignoreErrorOnOverflow) return;
      throw new IllegalArgumentException(
        "Paginator's current page " +
          page +
          " can't be beyond the last page " +
          this.lastPage +
          ".",
      );
    }

    if (page <= 0 && !this.flags.ignoreErrorOnUnderflow) {
      throw new IllegalArgumentException(
        "Tried to assign " +
          page +
          " to Paginator's current page," +
          "but it can't be lower than 1.",
      );
    }

    const previousCurrentPage = this.currentPage;
    this.currentPage = page;
    this.callEventListeners("page-change", previousCurrentPage, false);
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
          this.currentPage +
          ".",
      );

      return;
    }

    const previousCurrentPage = this.currentPage;
    this.currentPage--;
    this.callEventListeners("previous-page", previousCurrentPage, false);
  }

  public nextPage() {
    if (this.currentPage + 1 > this.lastPage) {
      console.warn(
        "Tried to go to the next page, but it doesn't exist. Current page is " +
          this.currentPage +
          " and last page is " +
          this.lastPage +
          ".",
      );

      return;
    }

    const previousCurrentPage = this.currentPage;
    this.currentPage++;
    this.callEventListeners("next-page", previousCurrentPage, false);
  }

  public getCurrentPage() {
    return this.currentPage;
  }

  public getLastPage() {
    return this.lastPage;
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

    return pages.map((page) => ({
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

  private callEventListeners(
    event: PaginatorEvent,
    previousCurrentPage?: number,
    hadOverflow?: boolean,
  ) {
    this.eventListeners.get(event)?.forEach((callback) => {
      callback(
        new PaginatorContext(
          this.currentPage,
          this.lastPage,
          hadOverflow ?? false,
          this,
          previousCurrentPage,
        ),
      );
    });
  }

  public countEventListeners(event: PaginatorEvent): number {
    return this.eventListeners.get(event)?.size ?? 0;
  }

  public resetEventListeners() {
    events.forEach((event) => {
      this.eventListeners.set(event, new Set());
    });
  }

  public getPaginationLinkForPage(
    page: number,
    { extraArgs, queryString }: GetPaginationParams = {},
  ): PaginationLink {
    queryString = this.getPaginationQueryString(queryString, extraArgs);

    return {
      page: this.currentPage,
      link: this.preparePaginationLink(queryString, page),
    };
  }

  private getPaginationQueryString(
    queryString: string | undefined,
    extraArgs: SearchRecord | undefined,
  ) {
    let searchParameters: SearchRecord = {};

    if (queryString) {
      searchParameters = PaginationPolitics.getQueryObjectFromUrl(queryString, searchParameters);
    }

    if (extraArgs) searchParameters = { ...searchParameters, ...extraArgs };

    if (this.pageQuery in searchParameters) delete searchParameters[this.pageQuery];

    return PaginationPolitics.getQueryStringFromObject(searchParameters);
  }

  private preparePaginationLink(queryString: string, page: number): string {
    if (queryString === "?") return `${this.url}?${this.pageQuery}=${page}`;
    return `${this.url + queryString}&${this.pageQuery}=${page}`;
  }
}
