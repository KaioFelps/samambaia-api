import { Head } from "@inertiajs/react";
import { ReactNode } from "react";
import { ToastContainer } from "react-toastify";

import { appConfig } from "@/config/app";

import { Footer } from "./footer";
import { Header } from "./header";
import { SideBar } from "./sideBar";

type PublicLayoutProps = {
  children: ReactNode;
};

export function PublicLayout({ children }: PublicLayoutProps) {
  return (
    <RawPublicLayout>
      <div className="main-screen-centralized flex gap-2">
        <SideBar />
        {children}
      </div>
    </RawPublicLayout>
  );
}

export function RawPublicLayout({ children }: PublicLayoutProps) {
  return (
    <>
      <Head>
        {/* Canonical link */}
        <link
          rel="canonical"
          href={appConfig.meta.appURL}
        />

        <meta
          name="description"
          content={appConfig.meta.description}
        />

        {/* Application name */}
        <meta
          name="application-name"
          content={appConfig.meta.title}
        />

        {/* Open Graph meta tags */}
        <meta
          property="og:title"
          content={appConfig.meta.title}
        />
        <meta
          property="og:description"
          content={appConfig.meta.description}
        />

        <meta
          property="og:type"
          content="website"
        />

        <meta
          property="og:url"
          content={appConfig.meta.appURL}
        />

        <meta
          property="og:image"
          content={appConfig.meta.cover}
        />

        <meta
          property="og:site_name"
          content={appConfig.meta.title}
        />

        <meta
          property="og:locale"
          content="pt_BR"
        />

        {/* Twitter meta tags */}
        <meta
          name="twitter:card"
          content="summary_large_image"
        />

        <meta
          name="twitter:title"
          content={appConfig.meta.title}
        />

        <meta
          name="twitter:description"
          content={appConfig.meta.description}
        />

        <meta
          name="twitter:image"
          content={appConfig.meta.cover}
        />
      </Head>
      <Header />
      {children}
      <Footer />
      <ToastContainer />
    </>
  );
}
