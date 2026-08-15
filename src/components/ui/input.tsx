import { forwardRef, type InputHTMLAttributes } from "react";
import { cn } from "@/lib/utils";

export const Input = forwardRef<HTMLInputElement, InputHTMLAttributes<HTMLInputElement>>(
  ({ className, ...props }, ref) => (
    <input
      ref={ref}
      className={cn(
        "h-9 w-full rounded-lg border border-white/10 bg-white/[0.035] px-3 text-sm text-white outline-none placeholder:text-zinc-600 focus:border-sky-400/60 focus:ring-2 focus:ring-sky-400/10",
        className,
      )}
      {...props}
    />
  ),
);
Input.displayName = "Input";
