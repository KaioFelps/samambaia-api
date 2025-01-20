import { Head } from "@inertiajs/react";
import { ReactNode } from "react";

import { Sprite } from "@/components/sprite";
import { RawPublicLayout } from "@/layouts/public";

type ErrorPageProps = {
  status: number;
};

export default function ErrorPage({ status }: ErrorPageProps) {
  const messages: Record<number, string> = {
    500: "Estamos com algum probleminha interno!",
    503: "Nossos serviços não estão disponíveis.",
    404: "Lugar errado?!",
    403: "Negativo! Acesso negado.",
    401: "Negativo! Não autorizado.",
  };

  const descriptions: Record<number, string> = {
    503: "Talvez alguma atualização esteja chegando? Quem sabe... Tente mais tarde.",
    500: "Não é você... Somos nós.",
    404: "Talvez você tenha ido longe demais! Dê alguns passos para trás.",
    403: "Eu posso. Você não pode! Você não pode, eu posso!",
    401: "Você não tem todo esse poder, se ponha no teu lugar.",
  };

  return (
    <>
      <Head title={messages[status]} />
      <main className="flex items-center justify-center gap-8 px-6 py-28">
        <div>
          <h1 className="text-purple-700 font-black text-5xl mb-4 max-w-96 text-balance">
            {messages[status] ?? messages[500]}
          </h1>
          <p className="text-2xl font-medium max-w-96 text-balance">
            {descriptions[status] ?? descriptions[500]}
          </p>
        </div>

        <Sprite
          x={-679}
          y={-118}
          width={119}
          height={180}
        />
      </main>
    </>
  );
}

ErrorPage.layout = (children: ReactNode) => <RawPublicLayout>{children}</RawPublicLayout>;
