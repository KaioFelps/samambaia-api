import "./app.scss";

import { colors } from "@crate/tailwind.config";
import { createInertiaApp } from "@inertiajs/react";
import { createRoot, hydrateRoot } from "react-dom/client";

import { appConfig } from "./config/app";
import { type PageComponent, resolvePageLayout, resolveTitle } from "./inertiaShared";

const appName = appConfig.appName ?? "Live Cosmic";
const production = import.meta.env.VITE_RUST_ENV === "PRODUCTION";

createInertiaApp({
  progress: { color: colors.purple[500], includeCSS: true },

  title: (title) => resolveTitle(title, appName),

  resolve: (name) => {
    const pages = import.meta.glob("./pages/**/*.tsx", { eager: true });
    const page = pages[`./pages/${name}.tsx`] as PageComponent;
    const resolvedPage = resolvePageLayout(page);

    return resolvedPage;
  },

  setup({ el, App, props }) {
    if (production) {
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
