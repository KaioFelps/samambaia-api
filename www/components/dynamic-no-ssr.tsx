import { type ComponentType, type ReactElement, useEffect, useState } from "react";

// biome-ignore lint/suspicious/noExplicitAny: gotta be any
type ComponentProps = ComponentType<any>;

type Props<P> = {
  fallback?: ReactElement | null;
  importFn: () => Promise<{ default: ComponentType<P> }>;
} & P;

function DynamicNoSsr<P>({ importFn, fallback = null, ...props }: Props<P>) {
  const [Component, setComponent] = useState<ComponentProps | null>(null);

  useEffect(() => {
    if (typeof window !== "undefined") {
      importFn()
        .then((module) => {
          setComponent(() => module.default);
        })
        .catch((err) => {
          console.error("Failed to load dynamic component:", err);
        });
    }
  }, [importFn]);

  if (!Component) {
    return fallback;
  }

  return <Component {...(props as unknown as P)} />;
}

export default DynamicNoSsr;
