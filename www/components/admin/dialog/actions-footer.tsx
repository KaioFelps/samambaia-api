type Props = {
  children: React.ReactNode;
  className?: string;
};

export function DialogActionsFooter({ children, className }: Props) {
  return (
    <div className={className}>
      <hr className="mb-4" />
      <div className="flex items-center justify-end gap-2">{children}</div>
    </div>
  );
}
