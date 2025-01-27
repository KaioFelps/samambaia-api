import { appConfig } from "@/config/app";

export const FaceGesture = {
  smile: "sml",
  angry: "agr",
  sad: "sad",
  surprised: "srp",
  speaking: "spk",
  noFace: "lol",
  normal: "std",
} as const;

export const CarriableItem = {
  wineGlass: 667,
  bubblingWater: 1,
  iceCream: 3,
  juicy: 5,
  carrot: 2,
  coffee: 6,
  soda: 44,
  bloodGlass: 43,
  lovePotion: 9,
  sushi: 42,
} as const;

type CarriableItemValue = typeof CarriableItem[keyof typeof CarriableItem];

export const Action = {
  sit: "sit",
  wave: "wav",
  walk: "wlk",
  lay: "lay",
  carrying: (item: CarriableItemValue) => `crr=${item}` as `crr=${CarriableItemValue}`,
  eating: (item: CarriableItemValue) => `drk=${item}` as `crr=${CarriableItemValue}`,
} as const;

type ActionKeys = Exclude<keyof typeof Action, "carrying" | "eating">;
type ActionType =
  typeof Action[ActionKeys] | `crr=${CarriableItemValue}` | `crr=${CarriableItemValue}`;

export interface ImagerParamsArguments {
  img_format: "png";
  direction: "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8";
  head_direction: "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8";
  size: "s" | "m" | "g";
  /* Use `Actions` enumeration to generate actions instead of hard-coding strings values. */
  action?: ActionType;
  gesture?: typeof FaceGesture[keyof typeof FaceGesture];
  headonly?: "0" | "1";
}

export abstract class Imager {
  public static getUserImage(
    nickname: string,
    params: Partial<ImagerParamsArguments> = {},
  ) {
    const searchParams = new URLSearchParams(Object.entries({ user: nickname, ...params }));
    return appConfig.imagerUrl + "?" + searchParams.toString();
  }
}

Object.freeze(FaceGesture);
Object.freeze(Action);
Object.freeze(CarriableItem);
