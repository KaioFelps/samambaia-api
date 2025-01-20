import { ReactNode } from "react";
import { ToastContainer } from "react-toastify";

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
  const topBg = "https://i.imgur.com/bBw1X5C.png";

  return (
    <>
      <Header topBg={topBg} />
      {children}
      <Footer />
      <ToastContainer />
    </>
  );
}
