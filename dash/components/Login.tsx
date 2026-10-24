import { IconCloudComputing } from '@tabler/icons-react';

function GithubIcon() {
  return (
    <svg
      aria-hidden="true"
      className="h-5 w-5"
      fill="currentColor"
      viewBox="0 0 24 24"
    >
      <path d="M12 .5a12 12 0 0 0-3.79 23.39c.6.11.82-.26.82-.58v-2.04c-3.34.73-4.04-1.61-4.04-1.61-.55-1.39-1.34-1.76-1.34-1.76-1.09-.75.08-.74.08-.74 1.2.08 1.84 1.23 1.84 1.23 1.07 1.83 2.8 1.3 3.49.99.11-.77.42-1.3.76-1.6-2.67-.3-5.47-1.34-5.47-5.93 0-1.31.47-2.38 1.23-3.22-.12-.3-.53-1.52.12-3.18 0 0 1-.32 3.3 1.23a11.5 11.5 0 0 1 6 0c2.3-1.55 3.3-1.23 3.3-1.23.65 1.66.24 2.88.12 3.18.77.84 1.23 1.91 1.23 3.22 0 4.6-2.8 5.62-5.48 5.92.43.37.81 1.1.81 2.22v3.29c0 .32.22.7.83.58A12 12 0 0 0 12 .5Z" />
    </svg>
  );
}

export default function Login() {
  return (
    <main className="flex min-h-screen items-center justify-center bg-[#121212] px-6 py-12 text-white">
      <section className="w-full max-w-md rounded-sm border border-white/10 bg-white p-8 shadow-2xl shadow-black/30 sm:p-10">
        <div className="flex justify-center">
          <IconCloudComputing stroke={1.5} className="h-16 w-16 text-[#5e29f0]" />
        </div>
        <div className="text-center mt-6">
          <h1 className="text-3xl text-black font-semibold tracking-tight">Quicksilicon</h1>
          <p className="mt-3 text-sm leading-6 text-[#404040]">
            Welcome back! Please sign in to continue
          </p>
        </div>

        <div className="mt-8 space-y-6">
          <button
            className="flex h-12 w-full items-center justify-center gap-3 rounded-sm text-black border border-[#ededf0] bg-[#fafafa] text-sm font-medium transition-colors hover:bg-white/10 focus:outline-none focus:ring-2 focus:ring-white/40"
            type="button"
          >
            <GithubIcon />
            Continue with Github
          </button>


        </div>
      </section>
    </main>
  );
}
