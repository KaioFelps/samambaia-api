import type { Icon } from "@phosphor-icons/react";
import { memo } from "react";
import Dialog from "@/components/admin/dialog";
import Button from "@/components/button";
import { IconButton } from "@/components/icon-button";
import type { JsonValue } from "@/types/audit-log";

type JsonModalButtonProps = {
  title: string;
  description?: string;
  icon: Icon;
  value: JsonValue | null | undefined;
};

export const JsonModalButton = memo(({ title, description, icon, value }: JsonModalButtonProps) => {
  if (value === null || value === undefined) return null;

  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <IconButton
          admin
          size="sm"
          variant="ghost"
          theme="info"
          icon={icon}
          aria-label={title}
          title={title}
        />
      </Dialog.Trigger>

      <Dialog.Content>
        <Dialog.Header title={title} description={description ?? `Conteúdo JSON de: ${title}`} />

        <Dialog.Container className="flex flex-col gap-4 text-gray-800">
          <pre className="max-h-[60vh] overflow-auto whitespace-pre-wrap wrap-break-word rounded-md bg-gray-100 p-4 text-xs">
            <code>{JSON.stringify(value, null, 2)}</code>
          </pre>

          <Dialog.ActionsFooter>
            <Dialog.Close asChild>
              <Button admin size="lg">
                Fechar
              </Button>
            </Dialog.Close>
          </Dialog.ActionsFooter>
        </Dialog.Container>
      </Dialog.Content>
    </Dialog.Root>
  );
});
