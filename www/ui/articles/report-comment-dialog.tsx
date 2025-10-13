import { useForm } from "@inertiajs/react";
import {
  type ButtonHTMLAttributes,
  type FormEvent,
  forwardRef,
  type ReactElement,
  useCallback,
  useId,
} from "react";
import { toast } from "react-toastify";
import PublicButton from "@/components/button/public-button";
import Dialog from "@/components/dialog";
import Form from "@/components/form";
import { routes } from "@/config/routes";

type Props = {
  trigger: ReactElement;
  commentId: string;
  open: boolean;
  setOpen: (value: boolean) => void;
} & ButtonHTMLAttributes<HTMLButtonElement>;

type ReportCommentFormData = {
  content: string;
};

export const ReportCommentDialog = forwardRef<HTMLButtonElement, Props>(
  ({ trigger, commentId, open, setOpen, ...props }, ref) => {
    const formId = useId();

    const { post, processing, errors, setData } = useForm<ReportCommentFormData>({ content: "" });

    const handleReportComment = useCallback(
      (e: FormEvent) => {
        e.preventDefault();
        post(routes.web.comment.report.create(commentId), {
          onSuccess: () => {
            toast.success("Denúncia realizada com sucesso.");
            setOpen(false);
          },
        });
      },
      [post, commentId, setOpen],
    );

    return (
      <Dialog.Root open={open} onOpenChange={setOpen}>
        <Dialog.Trigger asChild {...props} ref={ref}>
          {trigger}
        </Dialog.Trigger>

        <Dialog.Content>
          <Dialog.Header title="Denunciar comentário" />

          <form
            id={`report-comment-form-${formId}`}
            onSubmit={handleReportComment}
            className="mb-3">
            <Form.Input
              label="Detalhes"
              placeholder="Descreva o motivo da denúncia."
              type="text"
              validationError={errors.content}
              onInput={(event) => setData({ content: event.currentTarget.value })}
              required
            />
          </form>

          <div className="flex items-center gap-2 justify-end">
            <Dialog.Close asChild>
              <PublicButton.Default disabled={processing} aria-busy={processing} type="button">
                Cancelar
              </PublicButton.Default>
            </Dialog.Close>

            <PublicButton.Default
              disabled={processing}
              aria-busy={processing}
              type="submit"
              form={`report-comment-form-${formId}`}
              variant="yellow">
              Denunciar
            </PublicButton.Default>
          </div>
        </Dialog.Content>
      </Dialog.Root>
    );
  },
);
