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
import PublicForm from "@/components/form/public-form";
import { routes } from "@/config/routes";
import type { Comment } from "@/types/comment";

type Props = {
  trigger: ReactElement;
  comment: Comment;
  open: boolean;
  setOpen: (value: boolean) => void;
} & ButtonHTMLAttributes<HTMLButtonElement>;

type ReportCommentFormData = {
  content: string;
};

export const ReportCommentDialog = forwardRef<HTMLButtonElement, Props>(
  ({ trigger, comment, open, setOpen, ...props }, ref) => {
    const formId = useId();

    const { post, processing, errors, setData } = useForm<ReportCommentFormData>({ content: "" });

    const handleReportComment = useCallback(
      (e: FormEvent) => {
        e.preventDefault();
        post(routes.web.comment.report.create(comment.id), {
          onSuccess: () => {
            toast.success("Denúncia realizada com sucesso.");
            setOpen(false);
          },
        });
      },
      [post, comment, setOpen],
    );

    return (
      <Dialog.Root open={open} onOpenChange={setOpen}>
        <Dialog.Trigger asChild {...props} ref={ref}>
          {trigger}
        </Dialog.Trigger>

        <Dialog.Content>
          <Dialog.Header title="Denunciar comentário" />

          <PublicForm.Root
            id={`report-comment-form-${formId}`}
            onSubmit={handleReportComment}
            className="mb-3">
            <div>
              <PublicForm.Label asChild>
                <span>Autor do comentário</span>
              </PublicForm.Label>
              <p className="text-input disabled">{comment.author.nickname}</p>
            </div>

            <div>
              <PublicForm.Label asChild>
                <span>Comentário</span>
              </PublicForm.Label>
              <p className="text-input disabled">{comment.content}</p>
            </div>

            <PublicForm.Input
              label="Motivo da denúncia"
              placeholder="Descreva o motivo pelo qual deseja denunciar este comentário."
              type="text"
              validationError={errors.content}
              onInput={(event) => setData({ content: event.currentTarget.value })}
              required
            />
          </PublicForm.Root>

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
