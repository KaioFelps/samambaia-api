import { ReactNode } from "react";

import { Footer } from "./footer";
import { Header } from "./header";
import { SideBar } from "./sideBar";

type PublicLayoutProps = {
  children: ReactNode;
};

export function PublicLayout({ children }: PublicLayoutProps) {
  const topBg = "https://i.imgur.com/bBw1X5C.png";

  return (
    <>
      <Header topBg={topBg} />
      <div className="main-screen-centralized flex gap-2">
        <SideBar />
        {children}
      </div>
      <Footer />
    </>
  );
}
