import { Paginator, type PaginatorConstructorArgs } from ".";
import { getEmptyEventsMap, type PaginatorEvent, type PaginatorEventHandler } from "./events";
import type { PaginatorFlags } from "./flags";

export class PaginatorBuilder {
  private eventsHandlers: Map<PaginatorEvent, Set<PaginatorEventHandler>> = getEmptyEventsMap();
  private args: Partial<PaginatorConstructorArgs> = {};
  private flags: PaginatorFlags = {
    ignoreErrorOnOverflow: false,
    ignoreErrorOnUnderflow: false,
  };

  public constructor() {
    if (!this.args.url) {
      const url = window.location.origin + window.location.pathname;
      this.args.url = url;
    }
  }

  public ignoreOverflowErrors(): this {
    this.flags.ignoreErrorOnOverflow = true;
    return this;
  }

  public ignoreUnderflowErrors(): this {
    this.flags.ignoreErrorOnUnderflow = true;
    return this;
  }

  public setEventListener(event: PaginatorEvent, handler: PaginatorEventHandler) {
    this.eventsHandlers.get(event)?.add(handler);
  }

  public setAlign(align: Exclude<PaginatorConstructorArgs["align"], undefined>): this {
    this.args.align = align;
    return this;
  }

  public setLastPage(lastPage: number): this {
    this.args.lastPage = lastPage;
    return this;
  }

  public setCurrentPage(currentPage: number): this {
    this.args.currentPage = currentPage;
    return this;
  }

  public setPageQuery(pageQuery: string): this {
    this.args.pageQuery = pageQuery;
    return this;
  }

  public setVisibleButtons(visibleButtons: number): this {
    this.args.visibleButtons = visibleButtons;
    return this;
  }

  public registerEventListener(event: PaginatorEvent, handler: PaginatorEventHandler): this {
    this.eventsHandlers.get(event)?.add(handler);
    return this;
  }

  public build(): Paginator {
    this.validateRequiredArgs();
    this.tryCurrentPageFromSearchQuery();

    const paginator = new Paginator({
      url: this.args.url!,
      eventListeners: this.eventsHandlers,
      align: this.args.align,
      pageQuery: this.args.pageQuery,
      visibleButtons: this.args.visibleButtons,
      flags: this.flags,
    });

    // If there are any error, event listeners have a chance to handle it.
    paginator.setLastPage(this.args.lastPage!);
    if (this.args.currentPage) paginator.setCurrentPage(this.args.currentPage);

    return paginator;
  }

  private validateRequiredArgs() {
    if (!this.args.lastPage) throw new Error("Paginator builder requires a `lastPage` argument.");
  }

  private tryCurrentPageFromSearchQuery() {
    if (!this.args.url || !this.args.pageQuery) {
      return;
    }

    const url = new URL(this.args.url);
    const currentPage = url.searchParams.get(this.args.pageQuery);
    if (!currentPage) return;

    const currentPageAsNumber = Number(currentPage);
    if (Number.isNaN(currentPageAsNumber)) return;

    this.args.currentPage = currentPageAsNumber;
  }
}
