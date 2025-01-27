import { Head as InertiaHead } from "@inertiajs/react";
import React, { useMemo } from "react";

import { appConfig } from "@/config/app";
import { resolveTitle } from "@/inertiaShared";

type HeadProps = {
  title?: string;
  description?: string;
};

/**
    A wrapper around Inertia's Head component that handles more meta tags for provided title and
    description.
*/
export function Head({ title, description, children }: React.PropsWithChildren<HeadProps>) {
  const resolvedTitle = resolveTitle(title, appConfig.meta.title);

  return (
    <InertiaHead title={title}>
      {title && (
        <meta
          name="application-name"
          content={resolvedTitle}
        />
      )}

      {title && (
        <meta
          property="og:title"
          content={resolvedTitle}
        />
      )}

      {title && (
        <meta
          property="og:site_name"
          content={resolvedTitle}
        />
      )}

      {title && (
        <meta
          name="twitter:title"
          content={resolvedTitle}
        />
      )}

      {description && (
        <meta
          name="description"
          content={description}
        />
      )}

      {description && (
        <meta
          property="og:description"
          content={description}
        />
      )}

      {description && (
        <meta
          name="twitter:description"
          content={description}
        />
      )}

      {children}
    </InertiaHead>
  );
}
