import { toast } from "react-toastify";
import type { TinyMCE } from "tinymce";

export const copyHtmlToClipboard = async (editor: TinyMCE) => {
  const content = editor.activeEditor?.getContent();
  if (!content) {
    toast("Não há conteúdo a ser copiado.", { type: "error" });
    return;
  }

  await window.navigator.clipboard.writeText(content);
  toast("Conteúdo copiado com sucesso!", { type: "info" });
};
