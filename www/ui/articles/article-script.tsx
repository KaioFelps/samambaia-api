import { useEffect, useId } from "react";

type Props = {
  script: string;
};

const event: keyof WindowEventMap = "load";
const HAS_INJECTED_SCRIPT_KEY = "__clientside_script_has_been_injected";

export function ArticleScript({ script }: Props) {
  const scriptId = useId();
  useEffect(() => {
    const setHasInjectedScript = (value: boolean) => {
      (window as unknown as Record<string, unknown>)[HAS_INJECTED_SCRIPT_KEY] = value;
    };

    const hasInjectedScript = (): boolean => {
      const hasInjected = (window as unknown as Record<string, unknown>)[HAS_INJECTED_SCRIPT_KEY];
      return (hasInjected as boolean) ?? false;
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
    };
  }, [script, scriptId]);

  return null;
}
