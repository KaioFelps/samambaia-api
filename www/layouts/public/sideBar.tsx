import { usePage } from "@inertiajs/react";

import Popover from "@/components/popover";
import { Sprite } from "@/components/sprite";
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

      <div className="
        rounded-lg border-2 border-black/25 bg-white p-3 shadow-black/25
        shadow-[0_2px_0_0]
        "
      >
        <header className="section-header from-purple-500 to-purple-700 mb-3">
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
                  p-0.5 rounded-full outline-0 ring-0 ring-blue-500/25 focus-within:ring-4
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

              <Popover.Content
                title="Espera aí!"
                side="right"
              >
                Essa seção ainda está em desenvolvimento!
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
    </aside>
  );
}
