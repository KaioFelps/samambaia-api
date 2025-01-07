import { createInertiaApp } from "@inertiajs/react";
import createServer from "@inertiajs/react/server";
import ReactDOMServer from "react-dom/server";

const appName = process.env.APP_NAME ?? "Live Cosmic";

createServer(page =>
  createInertiaApp({
    page,

    title: (title) => (title
      ? `${appName} - ${title}`
      : appName),

    render: ReactDOMServer.renderToString,

    resolve: name => {
      const pages = import.meta.glob("./pages/**/*.tsx", { eager: true });
      return pages[`./pages/${name}.tsx`];
    },

    setup: ({ App, props }) => <App {...props} />,
  }),
);
