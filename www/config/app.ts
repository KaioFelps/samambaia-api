import cover from "@/assets/website-meta-cover.jpeg";

const appName = "Habblet Cosmic" as const;

export const appConfig = {
  appName,

  hostHotel: {
    name: "Habblet Hotel",
    shortName: "Habblet",
    baseUrl: "https://habblet.city",
  },

  assets: {
    topBg: "https://i.imgur.com/bBw1X5C.png",
    logo: "https://i.imgur.com/C7Lz4qH.png",
    adminLogo: "https://i.imgur.com/83zCYQD.png",
  },

  meta: {
    appURL: "https://live-cosmic-staging.squareweb.app/",
    title: appName,
    description: "Sua galáxia de entretenimento e informações no Habblet Hotel!",
    cover,
  },
} as const;

Object.freeze(appConfig);
