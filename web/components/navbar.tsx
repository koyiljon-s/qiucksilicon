"use client";

import { IconArrowUpRight } from '@tabler/icons-react';

const navItems = [
  { label: "Technology", href: "#technology" },
  { label: "Solutions", href: "#Solutions" },
  { label: "Whitepaper", href: "#white-paper" },
];

const rightItems = [{ label: "Get Started", href: "#start" }];

export default function Navbar() {

  return (
    <>
      <header className="sticky inset-x-0 top-0 z-50 bg-white backdrop-blur border-b border-[#d1d1d1]">
      <nav className="mx-auto flex max-w-7xl items-center justify-between border-r border-l border-[#d1d1d1]">
        <a
          href="#"
          className="flex items-center gap-1 text-[32px] font-bold tracking-tight text-[#010e21] px-8 py-4"
        >
          Quicksilicon
        </a>

        <ul className="mx-auto flex  items-center gap-12 justify-between">
          {navItems.map((item) => (
            <li key={item.href}>
              <a
                href={item.href}
                className="rounded-full px-4 py-2 text-lg font-normal text-[#010e21] transition-colors hover:text-[#ea34f7]"
              >
                {item.label}
              </a>
            </li>
          ))}
        </ul>

        <div className="hidden items-center md:flex px-8">
          {rightItems.map((item) => (
            <a
              key={item.href}
              href={item.href}
              className="py-2 px-6 text-xl text-white rounded-xl bg-linear-to-b from-[#548dff] to-[#2664eb] font-medium border border-[#75a3ff] ring ring-[#1949c2]"
            >
              {item.label}
            </a>
          ))}

        </div>
      </nav>
      </header>
    </>
  );
}