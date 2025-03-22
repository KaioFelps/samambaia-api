export type Pagination = {
  currentPage: number;
  totalItems: number;
  totalPages: number;
  itemsPerPage: number;
};

export type Paginated<T> = { data: T; pagination: Pagination };
