"use client";

const navItems = [
  { label: "Technology", href: "#technology" },
  { label: "Solutions", href: "#Solutions" },
  { label: "Whitepaper", href: "#white-paper" },
];

const rightItems = [{ label: "Get Started", href: "#start" }];

export default function Navbar() {

  return (
    <>
      <header className="fixed inset-x-0 top-0 z-50 bg-white backdrop-blur border-b border-[#d9d9d9]">
      <nav className="mx-auto flex max-w-384 items-center justify-between">
        <a
          href="#"
          className="flex items-center gap-1 text-[32px] font-bold tracking-tight text-[#010e21] border-r border-[#d9d9d9] px-10 py-4"
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

        <div className="hidden items-center md:flex pr-10">
          {rightItems.map((item) => (
            <a
              key={item.href}
              href={item.href}
              className="py-6 px-8  text-xl text-[#0a0a0a] bg-[#ff00ff] font-medium hover:bg-[#ecff6e]"
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