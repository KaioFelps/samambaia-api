import { PageProps } from "@inertiajs/core/types";

export default function AdminHome({ auth }: PageProps) {
  return (
    <main>
      <h1>Bem-vindo ao painel do cosmic, {auth?.user.nickname}!</h1>
    </main>
  );
}
