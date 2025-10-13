import { useForm } from "@inertiajs/react";
import clsx from "clsx";
import { useState } from "react";
import { toast } from "react-toastify";
import { Sprite } from "@/components/sprite";
import Tooltip from "@/components/tooltip";
import { routes } from "@/config/routes";
import { type Auth, Permission } from "@/types/auth";
import type { Comment } from "@/types/comment";
import { Imager } from "@/utils/imager";
import { DeleteCommentConfirmationDialog } from "./delete-comment-confirmation-dialog";
import { ReportCommentDialog } from "./report-comment-dialog";
import { ReportNeedsLoginDialog } from "./report-needs-login-dialog";

type Props = {
  auth?: Auth;
} & Comment;

export function CommentBox({ auth, ...comment }: Props) {
  const [reportDialogIsOpen, setReportDialogIsOpen] = useState(false);
  const { delete: formDelete, processing: isDeleting } = useForm();

  const userImage = Imager.getUserImage(comment.author.nickname, {
    size: "s",
    head_direction: "3",
  });

  const userCanDeleteComment =
    comment.author.nickname === auth?.user.nickname ||
    (auth?.permissions.includes(Permission.DeleteComment) ?? false);

  const handleDeleteComment = () => {
    formDelete(routes.web.comment.deleteComment(comment.id), {
      preserveScroll: true,
      onError: (errors) => {
        if ("error" in errors) toast.error(errors.error);
      },
      onSuccess: () => toast.success("Comentário deletado com sucesso!"),
    });
  };

  const reportCommentButton = (
    <button title="Denunciar" type="button">
      <Sprite width={16} height={16} x={-1} y={-65} />
    </button>
  );

  return (
    <div className="border-2 border-gray-700 rounded-md flex">
      <div
        className={clsx(
          "w-10 bg-yellow-500 flex flex-col justify-end rounded-ss-sm rounded-es-sm",
          "border-r-2 border-black/25",
          "shadow-[inset_0_3px_0_0_color-mix(in_oklab,_var(--color-white)_40%,_transparent),_inset_0_-2px_0_0_color-mix(in_oklab,_var(--color-black)_10%,_transparent)]",
        )}>
        <div
          style={{
            background: `url(${userImage}) -3px -10px`,
          }}
          className="pixelated w-10 min-h-14 h-auto"
        />
      </div>
      <div
        className={clsx(
          "px-1.5 py-0.5 flex-1 bg-gray-200 rounded-se-sm rounded-ee-sm flex flex-col justify-between",
          "shadow-[inset_0_3px_0_0_var(--color-white),_inset_0_-2px_0_0_color-mix(in_oklab,_var(--color-black)_10%,_transparent)]",
        )}>
        <p className="text-sm text-black">
          <span>{comment.author.nickname}: </span>
          {comment.content}
        </p>

        <div className="flex justify-between items-center">
          <span className="text-xs text-gray-700">
            {new Date(comment.createdAt).toLocaleDateString("pt-BR")}
          </span>

          <div className="flex items-center gap-1 pb-0.5">
            {userCanDeleteComment && (
              <Tooltip.Root>
                <Tooltip.Trigger asChild>
                  <DeleteCommentConfirmationDialog
                    buttonsShallBeDisabled={isDeleting}
                    trigger={
                      <button
                        title="Apagar comentário"
                        disabled={isDeleting}
                        aria-busy={isDeleting}
                        type="button">
                        <Sprite width={13} height={16} x={-130} y={-128} />
                      </button>
                    }
                    onConfirm={handleDeleteComment}
                  />
                </Tooltip.Trigger>

                <Tooltip.Container>
                  <Tooltip.Content>Apagar comentário</Tooltip.Content>
                </Tooltip.Container>
              </Tooltip.Root>
            )}

            <Tooltip.Root>
              <Tooltip.Trigger asChild>
                {auth ? (
                  <ReportCommentDialog
                    trigger={reportCommentButton}
                    open={reportDialogIsOpen}
                    setOpen={setReportDialogIsOpen}
                    commentId={comment.id}
                  />
                ) : (
                  <ReportNeedsLoginDialog trigger={reportCommentButton} />
                )}
              </Tooltip.Trigger>

              <Tooltip.Container>
                <Tooltip.Content>Denunciar comentário</Tooltip.Content>
              </Tooltip.Container>
            </Tooltip.Root>
          </div>
        </div>
      </div>
    </div>
  );
}
