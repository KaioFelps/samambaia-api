import { usePage } from "@inertiajs/react";

import { SharedProps } from "@/inertiaShared";

import { AnnouncementsSlider } from "./announcementsSlider";

export function SideBar() {
  const { announcements } = usePage<SharedProps>().props;

  return (
    <aside className="flex-1 max-w-[336px] flex flex-col gap-2">
      {
        announcements.data.length > 0 &&
          <AnnouncementsSlider announcements={announcements.data} />
      }
    </aside>
  );
}
