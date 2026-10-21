"use client"

import { LogoBurst } from "@/components/ui/logo-burst"

const settings = {
  color: "#D6D2CA",
  tentacleCount: 260,
  radius: 0.48,
  duration: 1.35,
  particleRatio: 0.62,
  breathe: true,
  replayOnClick: true,
}

// The stock filament for dark (#D6D2CA) and light (#3F3F46) pages. Either
// one means "follow the theme", so the burst swaps with light and dark mode.
const STOCK_COLORS = ["#D6D2CA", "#3F3F46"]

export default function LogoBurstDemo(props: Partial<typeof settings>) {
  const s = { ...settings, ...props }
  const autoColor = STOCK_COLORS.includes(s.color.trim().toUpperCase())

  return (
    <div className="flex min-h-[520px] w-full items-center justify-center bg-background p-4">
      <div className="relative h-[520px] w-full overflow-hidden rounded-xl bg-background">
        <LogoBurst
          color={autoColor ? undefined : s.color}
          tentacleCount={s.tentacleCount}
          radius={s.radius}
          duration={s.duration}
          particleRatio={s.particleRatio}
          breathe={s.breathe}
          replayOnClick={s.replayOnClick}
          // Replays the explosion whenever the duration changes.
          replayKey={s.duration}
          theme="auto"
        />
        {s.replayOnClick ? (
          <p className="pointer-events-none absolute inset-x-0 bottom-4 text-center text-xs text-muted-foreground">
            Click to replay
          </p>
        ) : null}
      </div>
    </div>
  )
}
