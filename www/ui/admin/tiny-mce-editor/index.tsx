// TinyMCE so the global var exists
import "tinymce/tinymce";
// DOM model
import "tinymce/models/dom/model";
// Theme
import "tinymce/themes/silver";
// Toolbar icons
import "tinymce/icons/default";
// Editor styles
import "tinymce/skins/ui/oxide/skin";
// importing the plugin js.
// if you use a plugin that is not listed here the editor will fail to load
import "tinymce/plugins/advlist";
import "tinymce/plugins/anchor";
import "tinymce/plugins/autolink";
import "tinymce/plugins/autoresize";
import "tinymce/plugins/autosave";
import "tinymce/plugins/charmap";
import "tinymce/plugins/code";
import "tinymce/plugins/codesample";
import "tinymce/plugins/directionality";
import "tinymce/plugins/emoticons";
import "tinymce/plugins/fullscreen";
import "tinymce/plugins/help";
import "tinymce/plugins/help/js/i18n/keynav/en";
import "tinymce/plugins/image";
import "tinymce/plugins/importcss";
import "tinymce/plugins/insertdatetime";
import "tinymce/plugins/link";
import "tinymce/plugins/lists";
import "tinymce/plugins/media";
import "tinymce/plugins/nonbreaking";
import "tinymce/plugins/pagebreak";
import "tinymce/plugins/preview";
import "tinymce/plugins/quickbars";
import "tinymce/plugins/save";
import "tinymce/plugins/searchreplace";
import "tinymce/plugins/table";
import "tinymce/plugins/visualblocks";
import "tinymce/plugins/visualchars";
import "tinymce/plugins/wordcount";
// importing plugin resources
import "tinymce/plugins/emoticons/js/emojis";
// Content styles, including inline UI like fake cursors
import "tinymce/skins/content/default/content";
import "tinymce/skins/ui/oxide/content";

import { Editor, type IAllProps } from "@tinymce/tinymce-react";
import clsx from "clsx";

import { ValidationErrorSpan } from "@/components/form/validation-error-alert";

type TinyMCEEditorProps = Omit<IAllProps, "licenseKey"> & {
  validationError?: string;
};

export function TinyMCEEditor({ validationError, ...props }: TinyMCEEditorProps) {
  return (
    <div className="article-content">
      <span className="block text-black text-sm ml-1 mb-2">
        Editor HTML
      </span>

      <ValidationErrorSpan validationError={validationError} />

      <Editor
        {...props}
        plugins={["code", "lists", "image", "anchor", "link", "quickbars", "autoresize", "table"]}
        toolbar={[
          { name: "extra", items: ["code"] },
          { name: "history", items: ["undo", "redo"] },
          {
            name: "styles",
            items: [
              "styles", "fontsize", "forecolor", "backcolor", "removeformat"],
          },
          {
            name: "formatting",
            items: [
              "bold",
              "italic",
              "underline",
              "strikethrough",
              "subscript",
              "superscript",
              "link",
              "unlink"],
          },
          {
            name: "alignment",
            items: [
              "alignleft", "aligncenter", "alignright", "alignjustify",
            ],
          },
          {
            name: "blocks",
            items: [
              "bullist", "numlist", "blockquote", "image", "table", "anchor",
            ],
          },
          { name: "indentation", items: ["outdent", "indent"] },
        ]}
        id="editor"
        init={{
          autoresize_bottom_margin: 0,
          language: "pt_BR",
          language_url: "/tiny-mce/langs/pt_BR.js",
          // if inline false, the editor is rendered inside an iFrame and page css files won't
          // be able to style the editor content. Not what we desire.
          inline: true,
          font_size_formats: clsx(
            "12px=var(--text-xs)",
            "14px=var(--text-sm)",
            "16px=var(--text-base)",
            "18px=var(--text-lg)",
            "20px=var(--text-xl)",
            "24px=var(--text-2xl)",
            "30px=var(--text-3xl)",
          ),
          line_height_formats: clsx(
            "leading-0=calc(var(--spacing)_*_0)",
            "leading-1=calc(var(--spacing)_*_3)",
            "leading-1.5=calc(var(--spacing)_*_5)",
            "leading-2=calc(var(--spacing)_*_7)",
          ),
          font_family_formats: clsx(
            "default=var(--font-default);Kanit=var(--font-default); Rowdies=var(--font-rowdies);",
            "Andale Mono=andale mono,times; Arial=arial,helvetica,sans-serif;",
            "Arial Black=arial black,avant garde; Book Antiqua=book antiqua,palatino;",
            "Comic Sans MS=comic sans ms,sans-serif; Courier New=courier new,courier;",
            "Georgia=georgia,palatino; Helvetica=helvetica; Impact=impact,chicago;",
            "Symbol=symbol; Tahoma=tahoma,arial,helvetica,sans-serif; Terminal=terminal,monaco;",
            "Times New Roman=times new roman,times; Trebuchet MS=trebuchet ms,geneva;",
            "Verdana=verdana,geneva; Webdings=webdings; Wingdings=wingdings,zapf dingbats",
          ),
          resize: true,
          verify_html: false,
          keep_styles: true,
          inline_styles: true,
          valid_children: "+body[style]",
          extended_valid_elements: "script[src|async|defer|type|charset],style,div[*],center",
          custom_elements: "style,script,center,div",
        }}
      />
    </div>
  );
}
