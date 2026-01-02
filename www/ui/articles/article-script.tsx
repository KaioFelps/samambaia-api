import { usePage } from "@inertiajs/react";
import { useEffect, useId, useMemo } from "react";
import type { ShowArticleProps } from "@/pages/articles/show";
import { decodeQuotes } from "@/utils/quotes";

type ExtendedWindow = Window &
  typeof globalThis & {
    [HAS_INJECTED_SCRIPT_KEY]: boolean | undefined;
  };

const event: keyof WindowEventMap = "load";
const HAS_INJECTED_SCRIPT_KEY = "__clientside_script_has_been_injected";

export function ArticleScript() {
  const page = usePage<ShowArticleProps>();
  const scriptId = useId();

  const script = useMemo(() => {
    const script = page.props.article.script?.trim() ?? "";
    return script === "" ? null : decodeQuotes(script);
  }, [page.props.article.script]);

  const cleanupScript = useMemo(() => {
    if (!script) return null;
    const cleanupScript = page.props.article.cleanupScript?.trim() ?? "";
    return cleanupScript === "" ? null : decodeQuotes(cleanupScript);
  }, [page.props.article.cleanupScript, script]);

  useEffect(() => {
    if (!page.props.article.script || !script) return;

    const setHasInjectedScript = (value: boolean) => {
      (window as ExtendedWindow)[HAS_INJECTED_SCRIPT_KEY] = value;
    };

    const hasInjectedScript = (): boolean => {
      const hasInjected = (window as ExtendedWindow)[HAS_INJECTED_SCRIPT_KEY];
      return hasInjected ?? false;
    };

    const injectScripts = () => {
      const scriptElement = document.createElement("script");
      scriptElement.textContent = script;
      scriptElement.type = "module";
      scriptElement.id = scriptId;

      document.body.appendChild(scriptElement);
    };

    const removeScripts = () => {
      const scriptElement = document.getElementById(scriptId);
      if (scriptElement) document.body.removeChild(scriptElement);
    };

    const injectScriptsOnce = () => {
      if (hasInjectedScript()) return;
      setHasInjectedScript(true);

      injectScripts();
      window.removeEventListener(event, injectScriptsOnce);
    };

    window.addEventListener(event, injectScriptsOnce);

    injectScriptsOnce();

    return () => {
      window.removeEventListener(event, injectScriptsOnce);
      removeScripts();
      setHasInjectedScript(false);

      if (cleanupScript) {
        const cleanup = new Function(cleanupScript);
        cleanup();
      }
    };
  }, [script, cleanupScript, scriptId, page]);

  return null;
}
