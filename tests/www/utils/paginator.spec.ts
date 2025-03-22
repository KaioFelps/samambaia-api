import { IllegalArgumentException } from "@/exceptions/illegal-argument-exception";
import { Paginator } from "@/utils/paginator";

const fakeUrl = "http://mylink.com";

describe("Paginator Specifications", () => {
  it("shouldn't be possible to set invalid values to a paginator instance", () => {
    expect(() => new Paginator({ url: fakeUrl, lastPage: -1 }))
      .toThrowError(IllegalArgumentException);

    expect(() => new Paginator({ url: fakeUrl, lastPage: 10, currentPage: 0 }))
      .toThrowError(IllegalArgumentException);

    let paginator = new Paginator({ url: fakeUrl, lastPage: 10 });

    expect(() => paginator.setCurrentPage(11)).toThrowError(IllegalArgumentException);
    expect(() => paginator.setCurrentPage(0)).toThrowError(IllegalArgumentException);
    expect(() => paginator.setCurrentPage(1)).not.toThrowError();

    expect(() => paginator.setLastPage(1)).not.toThrowError();
    expect(() => paginator.setLastPage(0)).toThrowError(IllegalArgumentException);

    expect(() => paginator.setPageQuery("")).toThrowError(IllegalArgumentException);
    expect(() => paginator.setPageQuery("p\n")).toThrowError(IllegalArgumentException);
    expect(() => paginator.setPageQuery("\r\np")).toThrowError(IllegalArgumentException);
    expect(() => paginator.setPageQuery("p age")).toThrowError(IllegalArgumentException);
    expect(() => paginator.setPageQuery("p")).not.toThrowError();

    expect(() => paginator.setVisibleButtons(0)).toThrowError(IllegalArgumentException);
    expect(() => paginator.setVisibleButtons(13)).toThrowError(IllegalArgumentException);
    expect(() => paginator.setVisibleButtons(11)).not.toThrowError();

    paginator = new Paginator({ url: fakeUrl, lastPage: 10, align: "left" });
    expect(() => paginator.setVisibleButtons(12)).not.toThrowError();
  });

  it("should be able to keep a centered pagination", () => {
    const paginator = new Paginator({ url: fakeUrl, lastPage: 10, align: "center" });
    paginator.setVisibleButtons(5);

    expect(paginator.getPagination()).toEqual(expect.arrayContaining([
      { page: 1, link: `${fakeUrl}?page=${1}` },
      { page: 2, link: `${fakeUrl}?page=${2}` },
      { page: 3, link: `${fakeUrl}?page=${3}` },
      { page: 4, link: `${fakeUrl}?page=${4}` },
      { page: 5, link: `${fakeUrl}?page=${5}` },
    ]));

    paginator.setCurrentPage(5);
    expect(paginator.getPagination()).toEqual(expect.arrayContaining([
      { page: 3, link: `${fakeUrl}?page=${3}` },
      { page: 4, link: `${fakeUrl}?page=${4}` },
      { page: 5, link: `${fakeUrl}?page=${5}` },
      { page: 6, link: `${fakeUrl}?page=${6}` },
      { page: 7, link: `${fakeUrl}?page=${7}` },
    ]));
  });

  it("should be able to keep a left-aligned paginatio", () => {
    const paginator = new Paginator({ url: fakeUrl, lastPage: 10, align: "left" });
    paginator.setVisibleButtons(6);

    expect(paginator.getPagination()).toEqual(expect.arrayContaining([
      { page: 1, link: `${fakeUrl}?page=${1}` },
      { page: 2, link: `${fakeUrl}?page=${2}` },
      { page: 3, link: `${fakeUrl}?page=${3}` },
      { page: 4, link: `${fakeUrl}?page=${4}` },
      { page: 5, link: `${fakeUrl}?page=${5}` },
      { page: 6, link: `${fakeUrl}?page=${6}` },
    ]));

    paginator.nextPage();
    paginator.nextPage();

    expect(paginator.getPagination()).toEqual(expect.arrayContaining([
      { page: 3, link: `${fakeUrl}?page=${3}` },
      { page: 4, link: `${fakeUrl}?page=${4}` },
      { page: 5, link: `${fakeUrl}?page=${5}` },
      { page: 6, link: `${fakeUrl}?page=${6}` },
      { page: 7, link: `${fakeUrl}?page=${7}` },
      { page: 8, link: `${fakeUrl}?page=${8}` },
    ]));
  });

  it("shouldn't let put even amount of visible buttons on center align", () => {
    let paginator = new Paginator({ url: fakeUrl, lastPage: 10, align: "center" });
    expect(() => paginator.setVisibleButtons(6)).toThrowError(IllegalArgumentException);

    paginator = new Paginator({ url: fakeUrl, lastPage: 10, align: "left" });
    expect(() => paginator.setVisibleButtons(6)).not.toThrowError();
  });

  it(
    "should not override existing search params but the page and the explicitly overriden ones",
    () => {
      const paginator = new Paginator({ url: fakeUrl, lastPage: 10 });

      expect(paginator.getPagination({ queryString: "?foo&bar=baz" })[0].link)
        .toMatch(`${fakeUrl}?foo&bar=baz&page=1`);
    });

  it("should let it change page query parameter key", () => {
    const paginator = new Paginator({ url: fakeUrl, lastPage: 10 });
    paginator.setPageQuery("p");

    expect(paginator.getPagination()[0].link)
      .toMatch(`${fakeUrl}?p=1`);
  });

  it("shouldn't move paginator cursor to out of bounds page", () => {
    const paginator = new Paginator({ url: fakeUrl, lastPage: 5 });

    expect(paginator.hasNextPage()).toBeTruthy();
    expect(paginator.hasPreviousPage()).toBeFalsy();

    paginator.previousPage();
    expect(paginator.getCurrentPage().page).toBe(1);

    paginator.nextPage();
    expect(paginator.hasNextPage()).toBeTruthy();
    expect(paginator.hasPreviousPage()).toBeTruthy();

    paginator.setCurrentPage(5);
    expect(paginator.hasNextPage()).toBeFalsy();
    expect(paginator.hasPreviousPage()).toBeTruthy();

    paginator.nextPage();
    expect(paginator.getCurrentPage().page).toBe(5);
  });

  it("should let register event listeners", () => {
    const [onGoToPrevPage, onGoToNextPage, onPageChange, anotherNextPageListener] = [
      vi.fn().mockImplementation((page: number) => console.log(`Went to prevv page: ${page}`)),
      vi.fn().mockImplementation((page: number) => console.log(`Went to next page: ${page}`)),
      vi.fn().mockImplementation((page: number) => console.log(`Page has changed to: ${page}`)),
      vi.fn().mockImplementation((_: number) => console.log("Another handler.")),
    ];

    const paginator = new Paginator({ url: fakeUrl, lastPage: 10 });
    paginator.addEventListener("next-page", onGoToNextPage);
    paginator.addEventListener("page-change", onPageChange);
    paginator.addEventListener("previous-page", onGoToPrevPage);
    paginator.addEventListener("next-page", anotherNextPageListener);

    expect(onGoToPrevPage).not.toBeCalled();
    expect(onGoToNextPage).not.toBeCalled();
    expect(onPageChange).not.toBeCalled();

    paginator.previousPage(); // must not count
    paginator.nextPage();
    paginator.nextPage();
    paginator.setCurrentPage(5);
    paginator.previousPage();
    paginator.setCurrentPage(10);
    paginator.nextPage(); // must not count

    expect(onGoToPrevPage).toBeCalledTimes(1);
    expect(onGoToNextPage).toBeCalledTimes(2);
    expect(anotherNextPageListener).toBeCalledTimes(2);
    expect(onPageChange).toBeCalledTimes(2);

    paginator.removeEventListener("next-page", onGoToNextPage);
    paginator.removeEventListener("page-change", onPageChange);
    paginator.removeEventListener("previous-page", onGoToPrevPage);

    paginator.previousPage();
    paginator.nextPage();
    paginator.setCurrentPage(1);

    expect(onGoToPrevPage).toBeCalledTimes(1);
    expect(onGoToNextPage).toBeCalledTimes(2);
    expect(onPageChange).toBeCalledTimes(2);

    expect(anotherNextPageListener).toBeCalledTimes(3);

    paginator.removeEventListener("next-page", anotherNextPageListener);

    paginator.nextPage();
    expect(anotherNextPageListener).toBeCalledTimes(3);

    expect(paginator.countEventListeners("next-page")).toBe(0);
    expect(paginator.countEventListeners("previous-page")).toBe(0);
    expect(paginator.countEventListeners("page-change")).toBe(0);

    paginator.addEventListener("next-page", onGoToNextPage);
    expect(paginator.countEventListeners("next-page")).toBe(1);

    paginator.resetEventListeners();
    expect(paginator.countEventListeners("next-page")).toBe(0);
  });
});
