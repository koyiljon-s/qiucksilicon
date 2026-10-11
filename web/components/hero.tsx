export default function Hero() {
  return (
    <section className="relative overflow-hidden bg-[#f285a0] flex-1 border-b border-[#3d2e32]">
      <div className="relative z-1 max-w-7xl mx-auto border-l border-[#3d2e32]">
        <div className="flex flex-col md:flex-row gap-8 items-center">
          <div className="flex-1 flex flex-col gap-8 py-20 md:py-70 px-4 md:px-10items-start text-left border-r border-[#3d2e32]">
            <div className="flex flex-col gap-4 items-start">
              <h1
                className="max-w-225  text-[#3d2d31] text-5xl sm:text-[60px] md:text-[60px] font-semibold tracking-normal"
              >
                Quicksilicon ZK Engine
              </h1>
              <p
                className="max-w-4xl font-medium text-[#3d2d31] text-4xl"
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

          <div className="flex-1" />
        </div>
      </div>
    </section>
  );
}