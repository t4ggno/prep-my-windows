import { cn } from "@/lib/utils";

const fieldClassName =
  "h-9 w-full rounded-lg border border-white/10 bg-white/[0.035] px-3 text-sm text-white outline-none placeholder:text-zinc-600 focus:border-sky-400/60 focus:ring-2 focus:ring-sky-400/10 aria-invalid:border-red-400/60";

export function Input({ className, ...props }: React.ComponentProps<"input">) {
  return <input className={cn(fieldClassName, className)} {...props} />;
}

export function Select({ className, ...props }: React.ComponentProps<"select">) {
  return <select className={cn(fieldClassName, "cursor-pointer pr-8", className)} {...props} />;
}
