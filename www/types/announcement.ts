export type Announcement = {
  id: string;
  url: string;
  image: string;
  external: boolean;
  description: string;
  created_at: Date | string;
  updated_at?: Date | string;
  author_id: string;
};

export type AnnouncementShort = {
  id: string;
  url: string;
  image: string;
  external: boolean;
  description: string;
};
