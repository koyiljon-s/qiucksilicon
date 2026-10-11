export default function Hero() {
  return (
    <section className="relative overflow-hidden bg-[#1c1c1a] flex-1">
      <div className="relative z-1 py-20 md:py-70 px-4 md:px-10 max-w-7xl mx-auto">
        <div className="flex flex-col gap-8 items-center mx-auto text-center">
          <div className="flex flex-col gap-4 items-center">
            <h1
              className="max-w-225  text-white text-5xl sm:text-[60px] md:text-[60px] font-semibold tracking-normal"
            >
              Quicksilicon ZK Engine
            </h1>
            <p
              className="max-w-4xl font-medium text-white text-4xl"
            >
              High-performance ZK proofs powered by modern GPUs
            </p>
          </div>

          <a
            href="/signup"
            className="inline-flex items-center bg-[#0d5dff] text-white rounded-xs font-medium px-4 py-3 no-underline hover:bg-[#D8B4FE] hover:text-[#0B0B0F] hover:shadow-[0_0_34px_rgba(216,180,254,0.3)] transition-colors"
          >
            Make The Infrastrucute Cloud
          </a>
        </div>
      </div>
    </section>
  );
}