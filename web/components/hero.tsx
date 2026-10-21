import { LogoBurst } from "@/components/ui/logo-burst";

export default function Hero() {
  return (
    <section className="relative overflow-hidden bg-white flex-1 border-b border-[#d1dce8]">
      <div className="relative z-1 max-w-384 mx-auto">
        <div className="flex flex-col md:flex-row items-center gap-6 px-12 md:gap-8 py-20 md:py-48 mx-auto">
          <div className="flex flex-1 flex-col items-center md:items-start text-center md:text-left">
            <div className="flex flex-col gap-4 items-center md:items-start">
              <h1
                className="max-w-225 text-[#191f29] text-[44px] sm:text-[60px] md:text-[60px] font-semibold tracking-normal leading-[1.3] "
              >
                Prove what is Real
              </h1>
              <p
                className="max-w-205 text-[#6b7685] text-[18px] md:text-[22px] leading-[1.4]"
              >
                Infrastructure built for developers who&apos;d rather ship than provision.
              </p>
            </div>

            <a
              href="/signup"
              className="inline-flex items-center bg-[#0d5dff] text-white rounded-sm text-base font-medium px-[25.6px] py-[10.4px] no-underline opacity-0 animate-[fadeUp_0.7s_ease_forwards] [animation-delay:240ms] hover:bg-[#D8B4FE] hover:text-[#0B0B0F] hover:shadow-[0_0_34px_rgba(216,180,254,0.3)] transition-colors"
            >
              Make The Infrastrucute Cloud
            </a>
          </div>

          <div className="flex shrink-0 flex-col">
            <h2 className="w-40 border-2 border-b-0 border-[#d9d9d9] bg-[#ededed] font-mono text-center text-lg font-semibold text-[#191f29]">
              Prove Anything
            </h2>
            <div className="relative h-175 md:h-155 w-full md:w-[540px] lg:w-[600px] overflow-hidden border-2 border-[#d9d9d9] bg-white">
              <LogoBurst theme="light" />
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
