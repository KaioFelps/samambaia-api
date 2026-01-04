import { ClipboardTextIcon } from "@phosphor-icons/react/dist/ssr/ClipboardText";
import { GavelIcon } from "@phosphor-icons/react/dist/ssr/Gavel";
import { GiftIcon } from "@phosphor-icons/react/dist/ssr/Gift";
import { HouseIcon } from "@phosphor-icons/react/dist/ssr/House";
import { NewspaperIcon } from "@phosphor-icons/react/dist/ssr/Newspaper";
import { StarIcon } from "@phosphor-icons/react/dist/ssr/Star";
import { TableIcon } from "@phosphor-icons/react/dist/ssr/Table";
import { TrophyIcon } from "@phosphor-icons/react/dist/ssr/Trophy";
import { UsersThreeIcon } from "@phosphor-icons/react/dist/ssr/UsersThree";
import { WarningIcon } from "@phosphor-icons/react/dist/ssr/Warning";
import { WrenchIcon } from "@phosphor-icons/react/dist/ssr/Wrench";
import * as Accordion from "@radix-ui/react-accordion";
import clsx from "clsx";
import { memo } from "react";
import { routes } from "@/config/routes";
import { SidebarMenuItem } from "./item";
import { SidebarMenuLink } from "./link";
import { SidebarMenuSection } from "./section";
import { SidebarSectionTitle } from "./sidebar-section-title";

export const SidebarMenu = memo(() => {
  return (
    <aside className={clsx("[grid-area:aside] ml-6 w-80")}>
      <div className="sticky top-6 max-h-full">
        <Accordion.Root type="multiple" className="flex flex-col gap-2">
          <SidebarMenuLink icon={HouseIcon} href="/gremio" label="Home" />

          {/* region: --- Conteúdo */}
          <SidebarSectionTitle title="Conteúdo" />

          <SidebarMenuSection
            label="Notícias"
            icon={NewspaperIcon}
            requires={{
              permissions: ["CreateArticle", "DeleteArticle", "UpdateArticle", "ApproveArticle"],
            }}>
            <SidebarMenuItem href={routes.admin.articles.list} label="Gerenciar notícias" />
            <SidebarMenuItem href={routes.admin.tags.list} label="Gerenciar tags" />
            <SidebarMenuItem
              href={routes.admin.articles.create}
              label="Nova notícia"
              requires="CreateArticle"
            />
            <SidebarMenuItem
              href={routes.admin.tags.create}
              label="Nova tag"
              requires="CreateArticleTag"
            />
          </SidebarMenuSection>

          <SidebarMenuSection label="Formulários" icon={TableIcon}>
            <Todo />
          </SidebarMenuSection>

          <SidebarMenuSection label="Emblemas grátis" icon={GiftIcon}>
            <Todo />
          </SidebarMenuSection>

          <SidebarMenuSection label="Moderação" icon={GavelIcon}>
            <Todo />
          </SidebarMenuSection>

          <SidebarSectionTitle title="Gerenciamento do site" />

          <SidebarMenuSection label="Membros destaque" icon={TrophyIcon}>
            <Todo />
          </SidebarMenuSection>

          <SidebarMenuSection label="Acervo" icon={ClipboardTextIcon}>
            <Todo />
          </SidebarMenuSection>

          <SidebarMenuSection label="Equipe" icon={UsersThreeIcon}>
            <Todo />
          </SidebarMenuSection>

          <SidebarMenuSection label="CMS" icon={StarIcon}>
            <Todo />
          </SidebarMenuSection>

          <SidebarSectionTitle title="Administração" />

          <SidebarMenuSection label="Usuários" icon={UsersThreeIcon}>
            <Todo />
          </SidebarMenuSection>

          <SidebarSectionTitle title="Promotoria" />

          <SidebarMenuSection label="Relatórios" icon={ClipboardTextIcon}>
            <Todo />
          </SidebarMenuSection>

          <SidebarMenuSection label="Pendências" icon={WrenchIcon}>
            <Todo />
          </SidebarMenuSection>
        </Accordion.Root>
      </div>
    </aside>
  );
});

function Todo() {
  return (
    <span
      className={clsx(
        "flex items-start bg-yellow-700/10 rounded-lg text-yellow-900 px-1 m-1 -mr-1 leading-7",
      )}>
      <WarningIcon size={20} weight="bold" className="mr-2 inline-block p-1 shrink-0 box-content" />
      Esta seção está em desenvolvimento.
    </span>
  );
}
