import { memo, useEffect, useId } from "react";
import { toast } from "react-toastify";

const event: keyof WindowEventMap = "load";

let HAS_INJECTED_SCRIPT = false;
let HAS_EXECUTED_SCRIPT = false;

type Props = {
  script: string | null;
  cleanupScript: string | null;
};

export const ArticleScript = memo(({ script, cleanupScript }: Props) => {
  const scriptId = useId();

  useEffect(() => {
    if (!script) return;

    const injectScripts = () => {
      const scriptElement = document.createElement("script");
      scriptElement.textContent = script;
      scriptElement.type = "module";
      scriptElement.id = scriptId;

      document.body.appendChild(scriptElement);
      HAS_INJECTED_SCRIPT = true;

      setTimeout(() => {
        HAS_EXECUTED_SCRIPT = true;
      }, 0);
    };

    const removeScripts = () => {
      const scriptElement = document.getElementById(scriptId);
      if (scriptElement) document.body.removeChild(scriptElement);
      HAS_INJECTED_SCRIPT = false;
    };

    const injectScriptsOnce = () => {
      window.removeEventListener(event, injectScriptsOnce);
      if (!HAS_INJECTED_SCRIPT) injectScripts();
    };

    window.addEventListener(event, injectScriptsOnce);

    injectScriptsOnce();

    return () => {
      window.removeEventListener(event, injectScriptsOnce);

      if (cleanupScript && HAS_EXECUTED_SCRIPT) {
        const cleanup = new Function(cleanupScript);
        try {
          cleanup();
        } catch (error) {
          console.error("An error occurred when running cleanup script", error);
          toast.error("Houve um problema ao rodar o script de limpeza desta notícia.");
        }
      }
      HAS_EXECUTED_SCRIPT = false;
      removeScripts();
    };
  });

  return null;
});
