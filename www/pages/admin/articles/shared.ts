import { toast } from "react-toastify";
import type { Editor } from "tinymce";

export const copyHtmlToClipboard = async (editor: Editor | null) => {
  const content = editor?.getContent();
  if (!content) {
    toast("Não há conteúdo a ser copiado.", { type: "error" });
    return;
  }

  await window.navigator.clipboard.writeText(content);
  toast("Conteúdo copiado com sucesso!", { type: "info" });
};
