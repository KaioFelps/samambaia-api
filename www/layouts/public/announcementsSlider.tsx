import "swiper/css";

import { colors } from "@crate/tailwind.config";
import type { Page } from "@inertiajs/core";
import { Link, router, usePage } from "@inertiajs/react";
import { useEffect, useState } from "react";
import { Autoplay } from "swiper/modules";
import { Swiper, SwiperSlide } from "swiper/react";

import { SharedProps } from "@/inertiaShared";
import { colorWithOpacity } from "@/lib/tailwind";
import { AnnouncementShort } from "@/types/announcement";

export function AnnouncementsSlider() {
  const page = usePage<SharedProps>();
  const [announcements, setAnnouncements] = useState<AnnouncementShort[]>();
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    setTimeout(() => {
      router.visit(page.url, {
        only: ["announcements"],
        fresh: true,
        onStart: () => { setIsLoading(true); },
        onFinish: () => { setIsLoading(false); },
        onSuccess(_page) {
          const page = _page as Page<SharedProps>;
          setAnnouncements(page.props.announcements.data);
        },
        onError(_errors) {
          setAnnouncements([]);
        },
      });
    }, 10);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  if (isLoading) return AnnouncementsSliderSkeleton();

  if (!announcements || !announcements?.length) return null;

  return (
    <Swiper
      spaceBetween={0}
      slidesPerView={1}
      loop
      modules={[Autoplay]}
      autoplay={{
        disableOnInteraction: false,
        delay: 5000,
        pauseOnMouseEnter: true,
      }}
      className="
        w-full bg-purple-700 aspect-square rounded-lg border-2 border-black
        shadow-black/25 shadow-[0_2px_0_0]
        "
    >
      {announcements.map(({ id, description, external, image, url }) => {
        const Anchor = external
          ? "a"
          : Link;

        return (
          <SwiperSlide key={"announcement-" + id}>
            <Anchor
              style={{
                backgroundImage: `url("${image}")`,
              }}
              className="
              pixelated flex flex-col items-center justify-end p-4 h-full w-full bg-center
              bg-cover
              "
              href={url}
              target={external
                ? "_blank"
                : "_self"}
              rel={external
                ? "noreferrer"
                : undefined}
            >
              <span
                style={{ textShadow: `0 3px 0 ${colorWithOpacity(colors.black, 25)}` }}
                className="
                px-6 py-1 font-bold bg-gray-800 rounded-full text-white text-2xl
                text-center text-balance shadow-black/25 shadow-[0_2px_0_0]
                "
              >
                {description}
              </span>
            </Anchor>
          </SwiperSlide>

        );
      })}
    </Swiper>
  );
}

function AnnouncementsSliderSkeleton() {
  return (
    <div className="
      w-full bg-purple-700 aspect-square rounded-lg border-2 border-black
      shadow-black/25 shadow-[0_2px_0_0] animate-pulse
      "
    />
  );
}
