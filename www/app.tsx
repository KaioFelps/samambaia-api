import "./app.css";

import { colors } from "@crate/tailwind.config";
import { createInertiaApp } from "@inertiajs/react";
import { ReactNode } from "react";
import { createRoot, hydrateRoot } from "react-dom/client";

import { type PageComponent, resolveTitle } from "./inertiaShared";
import { PublicLayout } from "./layouts/public";

const appName = import.meta.env.VITE_APP_NAME ?? "Live Cosmic";
const production = import.meta.env.VITE_RUST_ENV === "PRODUCTION";

createInertiaApp({
  progress: { color: colors.purple[500], includeCSS: true },

  title: (title) => resolveTitle(title, appName),

  resolve: (name) => {
    const pages = import.meta.glob("./pages/**/*.tsx", { eager: true });
    const page = pages[`./pages/${name}.tsx`] as PageComponent;

    page["default"].layout =
    page["default"].layout || ((page: ReactNode) => <PublicLayout>{page}</PublicLayout>);

    return page;
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
