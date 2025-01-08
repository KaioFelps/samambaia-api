import { ReactNode } from "react";

import { Footer } from "./footer";
import { Header } from "./header";

type PublicLayoutProps = {
  children: ReactNode
};

export function PublicLayout({ children }: PublicLayoutProps) {
  const topBg = "https://i.imgur.com/bBw1X5C.png";

  return (
    <>
      <Header topBg={topBg} />
      {children}
      <Footer />
    </>
  );
}
