import { PushPin } from "@phosphor-icons/react/dist/ssr/PushPin";
import clsx from "clsx";
import { memo } from "react";

import type { CouncilAlert } from "@/types/council-alert";

export const CouncilAlertCard = memo(({ content, pinned, title }: CouncilAlert) => {
  return (
    <article>
      <header className="flex justify-between py-3 border-b border-black/20">
        <h3 className="text-lg">{title}</h3>
        {pinned && <PushPin size={16} className="text-gray-700" />}
      </header>
      <div
        className={clsx(
          "py-3 flex flex-col gap-3",
          "[&*]:text-sm [&*]:font-light prose-p:leading-5",
          "prose-a:text-blue-500 prose-a:hover:text-blue-700",
          "prose-headings:font-bold prose-headings:mb-5 prose-headings:text-base prose-h1:text-lg",
        )}
        // biome-ignore lint/security/noDangerouslySetInnerHtml: it is safe to inject html here since its published by trusted authors
        dangerouslySetInnerHTML={{ __html: content }}
      />
    </article>
  );
});
