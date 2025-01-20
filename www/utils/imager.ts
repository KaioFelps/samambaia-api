import { appConfig } from "@/config/app";

export interface ImagerParamsArguments {
  img_format: "png";
  direction: "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8";
  head_direction: "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8";
  size: "s" | "m" | "g";
  action?: "sit" | "wav" | "drk";
  headonly?: "0" | "1";
}

export abstract class Imager {
  public static getUserImage(
    nickname: string,
    params: Partial<ImagerParamsArguments> = {},
  ) {
    const searchParams = new URLSearchParams(Object.entries({ ...params, user: nickname }));

    return appConfig.imagerUrl + "?" + searchParams.toString();
  }
}
