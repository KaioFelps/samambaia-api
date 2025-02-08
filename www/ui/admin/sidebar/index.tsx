import { ClipboardText } from "@phosphor-icons/react/dist/ssr/ClipboardText";
import { Gavel } from "@phosphor-icons/react/dist/ssr/Gavel";
import { Gift } from "@phosphor-icons/react/dist/ssr/Gift";
import { House } from "@phosphor-icons/react/dist/ssr/House";
import { Newspaper } from "@phosphor-icons/react/dist/ssr/Newspaper";
import { Star } from "@phosphor-icons/react/dist/ssr/Star";
import { Table } from "@phosphor-icons/react/dist/ssr/Table";
import { Trophy } from "@phosphor-icons/react/dist/ssr/Trophy";
import { UsersThree } from "@phosphor-icons/react/dist/ssr/UsersThree";
import { Warning } from "@phosphor-icons/react/dist/ssr/Warning";
import { Wrench } from "@phosphor-icons/react/dist/ssr/Wrench";
import * as Accordion from "@radix-ui/react-accordion";
import clsx from "clsx";
import { memo } from "react";

import { SidebarMenuItem } from "./item";
import { SidebarMenuLink } from "./link";
import { SidebarMenuSection } from "./section";
import { SidebarSectionTitle } from "./sidebarSectionTitle";

export const SidebarMenu = memo(() => {
  return (
    <Accordion.Root
      type="multiple"
      asChild
    >
      <aside className={clsx(
        "w-80 shrink-0 flex flex-col gap-2 sticky left-0 top-0",
      )}
      >
        <SidebarMenuLink
          icon={House}
          href="/gremio"
          label="Home"
        />

        {/* region: --- Conteúdo */}
        <SidebarSectionTitle title="Conteúdo" />

        <SidebarMenuSection
          label="Notícias"
          icon={Newspaper}
          requires={{
            permissions: ["CreateArticle", "DeleteArticle", "UpdateArticle", "ApproveArticle"],
          }}
        >
          <SidebarMenuItem
            href="/gremio/noticias"
            label="Gerenciar notícias"
          />
          <SidebarMenuItem
            href="/gremio/noticias/nova"
            label="Nova notícia"
            requires="CreateArticle"
          />
        </SidebarMenuSection>

        <SidebarMenuSection
          label="Formulários"
          icon={Table}
        >
          <Todo />
        </SidebarMenuSection>

        <SidebarMenuSection
          label="Emblemas grátis"
          icon={Gift}
        >
          <Todo />
        </SidebarMenuSection>

        <SidebarMenuSection
          label="Moderação"
          icon={Gavel}
        >
          <Todo />
        </SidebarMenuSection>

        {/* region: --- Gerenciamento do site */}
        <SidebarSectionTitle title="Gerenciamento do site" />

        <SidebarMenuSection
          label="Membros destaque"
          icon={Trophy}
        >
          <Todo />
        </SidebarMenuSection>

        <SidebarMenuSection
          label="Acervo"
          icon={ClipboardText}
        >
          <Todo />
        </SidebarMenuSection>

        <SidebarMenuSection
          label="Equipe"
          icon={UsersThree}
        >
          <Todo />
        </SidebarMenuSection>

        <SidebarMenuSection
          label="CMS"
          icon={Star}
        >
          <Todo />
        </SidebarMenuSection>

        {/* region: --- Administração */}
        <SidebarSectionTitle title="Administração" />

        <SidebarMenuSection
          label="Usuários"
          icon={UsersThree}
        >
          <Todo />
        </SidebarMenuSection>

        {/* region: --- Promotoria */}
        <SidebarSectionTitle title="Promotoria" />

        <SidebarMenuSection
          label="Relatórios"
          icon={ClipboardText}
        >
          <Todo />
        </SidebarMenuSection>

        <SidebarMenuSection
          label="Pendências"
          icon={Wrench}
        >
          <Todo />
        </SidebarMenuSection>
      </aside>
    </Accordion.Root>
  );
});

function Todo() {
  return (
    <span className={clsx(
      "flex items-start bg-yellow-700/10 rounded-lg text-yellow-900 px-1 m-1 -mr-1 leading-7",
    )}
    >
      <Warning
        size={20}
        weight="bold"
        className="mr-2 inline-block p-1 shrink-0 box-content"
      />
      Esta seção está em desenvolvimento.
    </span>
  );
}
