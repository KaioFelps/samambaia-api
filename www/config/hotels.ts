export const HotelsCode = {
  Habblet: "habblet",
  Habblive: "habblive",
} as const;

export type THotelsCode = (typeof HotelsCode)[keyof typeof HotelsCode];

export const hotelsConfig = {
  [HotelsCode.Habblet]: {
    imagerUrl: "https://www.habblet.city/habblet-imaging/avatarimage",
    apiBaseUrl: "https://api.habblet.city",
  },
  [HotelsCode.Habblive]: {
    imagerUrl: "https://habblive.in/imager.php",
    apiBaseUrl: "https://habblive.in/api/userinfo.php",
  },
};
