import { usePage } from "@inertiajs/react";

import iconsSprite from "@/assets/icons-sprite.png";
import Popover from "@/components/popover";
import { Sprite } from "@/components/sprite";
import { SharedProps } from "@/inertiaShared";
import { Imager } from "@/utils/imager";

import { AnnouncementsSlider } from "./announcementsSlider";

export function SideBar() {
  const { featuredUsers } = usePage<SharedProps>().props;

  return (
    <aside className="flex-1 max-w-[336px] flex flex-col gap-2">
      <AnnouncementsSlider />

      {/* GOSSIPING AND COMPLAINING */}
      <div className="card">
        <header className="section-header purple mb-3">
          <span>Reclame aqui</span>
        </header>

        <div>
          <label className="block mb-3">
            <input
              disabled
              type="text"
              placeholder="Fale mal de uma campanha e saia..."
              className="text-input"
            />
          </label>

          <div className="flex items-center gap-3 justify-end">
            <Popover.Root>

              <Popover.Trigger asChild>
                <button className="
                  p-0.5 rounded-full outline-0 ring-0 ring-blue-500/25 focus-visible:ring-4
                  transition-all duration-100 will-change-[box-shadow]
                  "
                >
                  <Sprite
                    x={-64}
                    y={-96}
                    width={18}
                    height={19}
                  />
                </button>
              </Popover.Trigger>

              <Popover.Content side="right">
                <Popover.Header title="Espera aí!" />
                <Popover.ContentContainer>
                  Essa seção ainda está em desenvolvimento!
                </Popover.ContentContainer>
              </Popover.Content>
            </Popover.Root>
            <button
              disabled
              className="btn flex items-center gap-2.5"
            >
              <Sprite
                x={-96}
                y={-64}
                width={11}
                height={19}
              />
              Reclamar
            </button>
          </div>

        </div>
      </div>

      {/* FEATURED USERS */}
      {featuredUsers.data.length > 0 && (
        <div className="card overflow-hidden">
          <header className="section-header purple">
            <span>Membro destaque</span>
          </header>

          <div className="flex flex-col gap-2">
            {featuredUsers.data.map(({ nickname, cause }) => (
              <div
                className="
                flex gap-3 [&:not(:last-child)]:overflow-hidden
                [&:not(:last-child)]:border-b-2 border-black/15
                "
                key={"live-cosmic-featured-user-" + nickname}
              >
                <div className="
                relative w-[150px] min-h-[172px] flex flex-col justify-end items-center -mb-4
                "
                >
                  <div
                    aria-hidden
                    className="w-[150px] h-[114px] absolute -bottom-[1px]"
                    style={{
                      backgroundImage: `url("${iconsSprite}")`,
                      backgroundPosition: "-570px 0",
                      backgroundRepeat: "no-repeat",
                    }}
                  />
                  <img
                    className="relative mb-16"
                    src={Imager.getUserImage(nickname, {
                      action: "wav",
                      head_direction: "3",
                      direction: "2",
                      size: "m",
                    })}
                    alt={`Habbo ${nickname} acenando`}
                  />
                </div>
                <div className="flex-1">
                  <span className="
                    text-2xl font-black leading-loose text-white highlight-member-name-stroke mb-1
                    "
                  >
                    {nickname}
                  </span>
                  <p className="text-sm">
                    {cause}
                  </p>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </aside>
  );
}
