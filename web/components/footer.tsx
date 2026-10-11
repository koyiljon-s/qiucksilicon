const columns = [
  {
    title: "Product",
    links: [
      { href: "/technology", label: "Technology" },
      { href: "/solution", label: "Solution" },
      { href: "/documentation", label: "Documentation" },
    ],
  },
];

export default function Footer() {
  return (
    <footer className="bg-white border-t border-[#eaeaea]">
      <div className="px-4 md:px-10 py-12 max-w-7xl mx-auto">
        <div className="flex flex-col md:flex-row gap-10 justify-between">
          <a
            href="/"
            className="text-xl font-semibold text-[#171717] no-underline"
          >
            Quicksilicon
          </a>

          <div className="flex flex-col sm:flex-row gap-10">
            {columns.map((column) => (
              <div key={column.title} className="flex flex-col gap-3">
                <span className="text-sm font-semibold text-[#171717]">
                  {column.title}
                </span>
                {column.links.map((link) => (
                  <a
                    key={link.href}
                    href={link.href}
                    className="text-sm font-medium text-[#171717] no-underline hover:text-[#0d5dff] transition-colors"
                  >
                    {link.label}
                  </a>
                ))}
              </div>
            ))}
          </div>
        </div>

        <p className="mt-10 text-sm text-[#171717]">
          © 2026 Quicksilicon. All rights reserved.
        </p>
      </div>
    </footer>
  );
}
