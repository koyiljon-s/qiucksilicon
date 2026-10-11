const links = [
  { href: "/technology", label: "Technology" },
  { href: "/solution", label: "Solution" },
  { href: "/documentation", label: "Documentation" },
];

export default function Navbar() {
  return (
    <header className="relative bg-white border-b border-[#3d2e32]">
      <nav className="flex items-center justify-between py-3 max-w-7xl mx-auto">
        <a
          href="/"
          className="flex-1 text-2xl font-semibold text-[#171717] no-underline"
        >
          Quicksilicon
        </a>

        <div className="flex items-center justify-center gap-8">
          {links.map((link) => (
            <a
              key={link.href}
              href={link.href}
              className="text-base font-medium text-[#171717] no-underline hover:text-[#0d5dff] transition-colors"
            >
              {link.label}
            </a>
          ))}
        </div>

        <div className="flex-1 flex items-center justify-end">
          <a
            href="/get-started"
            className="inline-flex items-center bg-[#ff00ff] text-white rounded-xs font-medium px-4 py-2 no-underline hover:bg-[#D8B4FE] hover:text-[#0B0B0F] hover:shadow-[0_0_34px_rgba(216,180,254,0.3)] transition-colors"
          >
            Get started
          </a>
        </div>
      </nav>
    </header>
  );
}
