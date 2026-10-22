import { cn } from "@/lib/utils";

type HatchDividerProps = {
  className?: string;
};

export default function HatchDivider({ className }: HatchDividerProps) {
  return (
    <div
      aria-hidden="true"
      className={cn(
        "pointer-events-none w-full border-y border-[#d1d1d1]",
        className,
      )}
    >
      <div className="relative mx-auto h-18 w-full max-w-7xl">
        {/* hatch */}
        <div className="h-full w-full bg-[repeating-linear-gradient(-60deg,#d1d1d1,#d1d1d1_1px,transparent_1px,transparent_10px)]" />

        {/* vertical guide lines */}
        <span className="absolute -top-10 -bottom-10 left-0 w-px bg-[#d1d1d1]" />
        <span className="absolute -top-10 -bottom-10 right-0 w-px bg-[#d1d1d1]" />

        {/* "+" markers */}
        <span className="absolute left-0 top-0 -translate-x-1/2 -translate-y-1/2 font-mono text-lg leading-none text-[#525252]">
          +
        </span>
        <span className="absolute right-0 top-0 translate-x-1/2 -translate-y-1/2 font-mono text-lg leading-none text-[#525252]">
          +
        </span>
      </div>
    </div>
  );
}