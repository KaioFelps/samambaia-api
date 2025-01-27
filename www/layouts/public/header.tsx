import { colors } from "@crate/tailwind.config";
import { Link } from "@inertiajs/react";

import Dropdown from "@/components/dropdown";

import { Sprite } from "../../components/sprite";
import { colorWithOpacity } from "../../lib/tailwind";
import { NavDropdownContent, NavDropdownItem } from "./navDropDownContent";
import { UserBox } from "./userBox";

type HeaderProps = {
  topBg: string;
};

export function Header({ topBg }: HeaderProps) {
  return (
    <header className="mb-2">
      <div
        style={{ backgroundImage: `url("${topBg}")` }}
        className="
            pixelated w-full h-[292px] bg-center animate-top-bg grid place-items-center
          "
      >
        <Link href="/">
          <img
            src="https://i.imgur.com/C7Lz4qH.png"
            alt="Live Cosmic"
            className="pixelated"
          />
        </Link>
      </div>

      <div
        style={{
          boxShadow: `inset 0 2px 0 0 ${colorWithOpacity(colors.black)},
            inset 0 4px 0 0 ${colorWithOpacity(colors.white, 15)},
            inset 0 -2px 0 0 ${colorWithOpacity(colors.black)},
            0 2px 0 0 ${colorWithOpacity(colors.black, 20)}`,
        }}
        className="bg-gray-800 text-white"
      >
        <div className="h-[68px] main-screen-centralized mx-auto flex items-center justify-between">

          <nav className="flex h-full">
            <Link
              href="/"
              className="navbar-item"
            >
              <span className="block size-[40px]">
                <Sprite
                  x={-65}
                  y={-2}
                  width={38}
                  height={39}
                />
              </span>
              Início
            </Link>

            <Dropdown.Root>
              <Dropdown.Trigger className="navbar-item group">
                <span className="block size-[40px]">
                  <Sprite
                    x={-128}
                    y={0}
                    width={36}
                    height={40}
                  />
                </span>
                Cosmic

                <NavBarDropdownIndicator />
              </Dropdown.Trigger>

              <NavDropdownContent>
                <NavDropdownItem href="">Equipe</NavDropdownItem>
                <NavDropdownItem href="">História</NavDropdownItem>
                <NavDropdownItem href="">Padrão de Excelência</NavDropdownItem>
              </NavDropdownContent>
            </Dropdown.Root>

            <Link
              href=""
              className="navbar-item"
            >
              <span className="block size-[40px]">
                <Sprite
                  x={0}
                  y={0}
                  width={40}
                  height={40}
                />
              </span>
              Jornalismo
            </Link>
            <Link
              href=""
              className="navbar-item"
            >
              <span className="block size-[40px]">
                <Sprite
                  x={-258}
                  y={0}
                  width={37}
                  height={39}
                />
              </span>
              Habblive
            </Link>
            <Link
              href=""
              className="navbar-item"
            >
              <span className="block size-[40px]">
                <Sprite
                  x={-195}
                  y={-1}
                  width={33}
                  height={39}
                />
              </span>
              VIP
            </Link>
          </nav>

          <UserBox />
        </div>

      </div>
    </header>
  );
}

function NavBarDropdownIndicator() {
  return (
    <Sprite
      x={-184}
      y={-63}
      height={19}
      width={14}
      className="rotate-90 group-data-[state=open]:-rotate-90 transition-[transform] duration-150"
    />
  );
}
