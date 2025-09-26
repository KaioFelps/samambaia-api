import { appConfig } from "@/config/app";
import { ImagerFactory } from "@/core/imager/factory";
import type { Imager } from "@/core/imager/imager";

const imager = Object.freeze(ImagerFactory.create(appConfig.hostHotel.code));

export function getImagerInstance(): Readonly<Imager> {
  return imager;
}
