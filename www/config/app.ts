import cover from "@/assets/website-meta-cover.jpeg";

const appName = "Live Cosmic" as const;

export const appConfig = {
  appName,
  imagerUrl: "https://habblive.in/imager.php",
  userInfoUrl: "https://habblive.in/api/userinfo.php",

  meta: {
    appURL: "https://live-cosmic-staging.squareweb.app/",
    title: appName,
    description: "Sua galáxia de entretenimento e informações no Habblive Hotel!",
    cover,
  },
} as const;

Object.freeze(appConfig);
