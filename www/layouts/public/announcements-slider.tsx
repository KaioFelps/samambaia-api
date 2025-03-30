import "swiper/css";

import { type Page } from "@inertiajs/core";
import { Link, router, usePage } from "@inertiajs/react";
import React, { memo, useEffect, useState } from "react";
import { Autoplay } from "swiper/modules";
import { Swiper, SwiperSlide } from "swiper/react";

import { AnnouncementShort } from "@/types/announcement";

export const AnnouncementsSlider = memo(() => {
  const page = usePage();
  const [announcements, setAnnouncements] = useState<AnnouncementShort[] | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (announcements) return;
    setTimeout(() => {
      router.get(page.url, {}, {
        only: ["announcements"],
        onStart: () => { setIsLoading(true); },
        onFinish: () => { setIsLoading(false); },
        onSuccess(_page) {
          const page = _page as Page;
          setAnnouncements(page.props.announcements.data);
        },
        onError(_errors) {
          setAnnouncements([]);
        },
      });
    }, 0);
  }, [announcements, page.url]);

  if (isLoading) return <AnnouncementsSliderSkeleton />;

  if (!announcements || !announcements?.length) return null;

  return (
    <MemoizedSwiper amount={announcements.length}>
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
                style={{
                  textShadow: "0 3px 0 color-mix(in oklab, var(--color-black) 25%, transparent)",
                }}
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
    </MemoizedSwiper>
  );
});

const AnnouncementsSliderSkeleton = memo(() => (
  <div className="
    w-full bg-purple-700 aspect-square rounded-lg border-2 border-black
    shadow-black/25 shadow-[0_2px_0_0] animate-pulse
    "
  />
));

const MemoizedSwiper = memo(({ children, amount }: React.PropsWithChildren<{ amount: number }>) => (
  <Swiper
    spaceBetween={0}
    slidesPerView={1}
    loop={amount > 1}
    modules={amount > 1
      ? [Autoplay]
      : undefined}
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
    {children}
  </Swiper>
));
