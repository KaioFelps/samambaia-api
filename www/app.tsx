import "@/styles/app.scss";

import { colors } from "@crate/tailwind.config";
import { createInertiaApp } from "@inertiajs/react";
import { createRoot, hydrateRoot } from "react-dom/client";

import { appConfig } from "./config/app";
import { pageResolver, resolveTitle } from "./lib/inertia";

const appName = appConfig.appName;

createInertiaApp({
  progress: { color: colors.purple[500], includeCSS: true },

  title: (title) => resolveTitle(title, appName),

  resolve: pageResolver,

  setup({ el, App, props }) {
    const isSSR = document.head
      .querySelector("meta[name='ssr']")
      ?.getAttribute("content") === "true";

    if (isSSR) {
      hydrateRoot(
        el,
        <App {...props} />,
      );
    } else {
      createRoot(el).render(
        <App {...props} />,
      );
    }
  },
});
