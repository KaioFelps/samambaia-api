import "./app.css";

import config from "@crate/tailwind.config";
import { createInertiaApp } from "@inertiajs/react";
import { createRoot, hydrateRoot } from "react-dom/client";

const appName = import.meta.env.VITE_APP_NAME ?? "Live Cosmic";
const production = import.meta.env.VITE_RUST_ENV === "PRODUCTION";

createInertiaApp({
  progress: { color: config.theme.colors["purple-500"], includeCSS: true },
  title: (title) => (title
    ? `${appName} - ${title}`
    : appName),

  resolve: (name) => {
    const pages = import.meta.glob("./pages/**/*.tsx", { eager: true });
    return pages[`./pages/${name}.tsx`];
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
