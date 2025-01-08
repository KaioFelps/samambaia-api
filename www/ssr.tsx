import { createInertiaApp } from "@inertiajs/react";
import createServer from "@inertiajs/react/server";
import { ReactNode } from "react";
import ReactDOMServer from "react-dom/server";

import { PageComponent, resolveTitle } from "./inertiaShared";
import { PublicLayout } from "./layouts/public";

const appName = process.env.APP_NAME ?? "Live Cosmic";

createServer(page =>
  createInertiaApp({
    page,

    title: (title) => resolveTitle(title, appName),

    render: ReactDOMServer.renderToString,

    resolve: name => {
      const pages = import.meta.glob("./pages/**/*.tsx", { eager: true });
      const page = pages[`./pages/${name}.tsx`] as PageComponent;

      page.default.layout =
      page.default.layout || ((page: ReactNode) => <PublicLayout>{page}</PublicLayout>);

      return page;
    },

    setup: ({ App, props }) => <App {...props} />,
  }),
);
