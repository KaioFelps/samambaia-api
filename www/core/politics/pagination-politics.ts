export type SearchRecord = Record<string, string | number | undefined>;

export type GetVisibleButtonsParams = {
  currentPage: number;
  lastPage: number;
  visibleButtons: number;
  align: "left" | "center";
};

export class PaginationPolitics {
  public static DEFAULT_VISIBLE_BUTTONS = 5;
  public static MAX_VISIBLE_BUTTONS = 12;

  public static getVisibleButtons({
    align,
    lastPage,
    currentPage,
    visibleButtons,
  }: GetVisibleButtonsParams) {
    if (align === "center") {
      return PaginationPolitics.getCenteredVisibleButtons(currentPage, lastPage, visibleButtons);
    }

    return PaginationPolitics.getLeftAlignedVisibleButtons(currentPage, lastPage, visibleButtons);
  }

  private static getCenteredVisibleButtons(
    currentPage: number,
    lastPage: number,
    visibleButtons: number,
  ) {
    let maxLeft = currentPage - Math.floor(visibleButtons / 2);
    let maxRight = currentPage + Math.floor(visibleButtons / 2);

    if (maxLeft <= 1) {
      maxLeft = 1;
      maxRight = visibleButtons;
    }

    if (maxRight >= lastPage) {
      maxLeft = lastPage - (visibleButtons - 1);
      maxRight = lastPage;
    }

    if (lastPage < visibleButtons + 1) {
      maxLeft = 1;
      maxRight = lastPage;
    }

    return {
      maxLeft,
      maxRight,
    };
  }

  private static getLeftAlignedVisibleButtons(
    currentPage: number,
    lastPage: number,
    visibleButtons: number,
  ) {
    let maxLeft = currentPage;
    let maxRight = currentPage + visibleButtons;

    if (maxRight > lastPage) {
      const bypassed = maxRight - lastPage;
      maxRight = lastPage;
      maxLeft -= bypassed;
    }

    if (maxLeft < 1) maxLeft = 1;

    return { maxLeft, maxRight };
  }

  public static getQueryObjectFromUrl(url: string, args: SearchRecord = {}) {
    let queryString = url;

    if (queryString.indexOf("?") === -1) {
      return {};
    }

    queryString = queryString.substring(queryString.indexOf("?") + 1);

    return Object.assign(Object.fromEntries(new URLSearchParams(queryString)), args);
  }

  public static getQueryStringFromObject(args: SearchRecord = {}) {
    const params = Object.entries(args).map(([key, value]) => {
      if (value) return `${key}=${value}`;
      return key;
    });

    const queryString = `?${params.join("&")}`;

    return queryString;
  }
}
