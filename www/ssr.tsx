import { createInertiaApp } from "@inertiajs/react";
import createServer from "@inertiajs/react/server";
import ReactDOMServer from "react-dom/server";

import { appConfig } from "./config/app";
import { pageResolver, resolveTitle } from "./lib/inertia";

const appName = appConfig.appName ?? "Live Cosmic";

createServer(page =>
  createInertiaApp({
    page,

    title: (title) => resolveTitle(title, appName),

    render: ReactDOMServer.renderToString,

    resolve: pageResolver,

    setup: ({ App, props }) => <App {...props} />,
  }),
);
