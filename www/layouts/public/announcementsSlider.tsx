import "swiper/css";

import { colors } from "@crate/tailwind.config";
import { Link } from "@inertiajs/react";
import { Autoplay } from "swiper/modules";
import { Swiper, SwiperSlide } from "swiper/react";

import { colorWithOpacity } from "@/lib/tailwind";
import { AnnouncementShort } from "@/types/announcement";

type AnnouncementsSliderProps = {
  announcements: AnnouncementShort[];
};

export function AnnouncementsSlider({ announcements }: AnnouncementsSliderProps) {
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
          w-full bg-purple-700 h-[282px] rounded-lg border-2 border-black
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
                pixelated flex flex-col items-center justify-end p-6 h-full w-full bg-center
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
                  px-[20px] py-1 font-bold bg-gray-800 rounded-full text-white text-2xl
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
