import { createInertiaApp } from "@inertiajs/react";
import createServer from "@inertiajs/react/server";
import ReactDOMServer from "react-dom/server";

import { type PageComponent, resolvePageLayout, resolveTitle } from "./inertiaShared";

const appName = process.env.APP_NAME ?? "Live Cosmic";

createServer(page =>
  createInertiaApp({
    page,

    title: (title) => resolveTitle(title, appName),

    render: ReactDOMServer.renderToString,

    resolve: name => {
      const pages = import.meta.glob("./pages/**/*.tsx", { eager: true });
      const page = resolvePageLayout(pages[`./pages/${name}.tsx`] as PageComponent);

      return page;
    },

    setup: ({ App, props }) => <App {...props} />,
  }),
);
