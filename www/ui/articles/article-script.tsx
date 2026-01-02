import { usePage } from "@inertiajs/react";
import { useEffect, useId, useMemo } from "react";
import { toast } from "react-toastify";
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

      setTimeout(() => {
        setHasInjectedScript(true);
      }, 0);
    };

    const removeScripts = () => {
      const scriptElement = document.getElementById(scriptId);
      if (scriptElement) document.body.removeChild(scriptElement);
      setHasInjectedScript(false);
    };

    const injectScriptsOnce = () => {
      window.removeEventListener(event, injectScriptsOnce);
      if (!hasInjectedScript()) injectScripts();
    };

    window.addEventListener(event, injectScriptsOnce);

    injectScriptsOnce();

    return () => {
      window.removeEventListener(event, injectScriptsOnce);

      if (cleanupScript && hasInjectedScript()) {
        const cleanup = new Function(cleanupScript);
        try {
          cleanup();
        } catch (error) {
          console.error("An error occurred when running cleanup script", error);
          toast.error("Houve um problema ao rodar o script de limpeza desta notícia.");
        }
      }

      removeScripts();
    };
  }, [script, cleanupScript, scriptId, page]);

  return null;
}
