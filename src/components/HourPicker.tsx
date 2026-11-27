import { useState } from "react";
import * as Popover from "@radix-ui/react-popover";
import { ChevronDown, Clock3 } from "lucide-react";
import { HOURS, hourLabel } from "@/lib/activeHours";
import { cn } from "@/lib/utils";

export function HourPicker({
  value,
  label,
  disabled,
  className,
  isAllowed = () => true,
  isHighlighted = () => false,
  onChange,
}: {
  value: number;
  label: string;
  disabled?: boolean;
  className?: string;
  isAllowed?: (hour: number) => boolean;
  isHighlighted?: (hour: number) => boolean;
  onChange: (hour: number) => void;
}) {
  const [open, setOpen] = useState(false);

  return (
    <Popover.Root open={open} onOpenChange={setOpen}>
      <Popover.Trigger asChild>
        <button className={cn("hour-picker-trigger", className)} aria-label={label} disabled={disabled}>
          <Clock3 className="size-3.5 text-sky-300/80" />
          <span>{hourLabel(value)}</span>
          <ChevronDown className="ml-auto size-3.5 text-zinc-500" />
        </button>
      </Popover.Trigger>
      <Popover.Portal>
        <Popover.Content className="hour-picker" align="end" sideOffset={6} aria-label={label}>
          <div className="hour-picker-grid">
            {HOURS.map((hour) => (
              <button
                key={hour}
                className={cn(hour === value && "selected", isHighlighted(hour) && "in-range")}
                disabled={!isAllowed(hour)}
                onClick={() => {
                  setOpen(false);
                  if (hour !== value) onChange(hour);
                }}
              >
                {hourLabel(hour)}
              </button>
            ))}
          </div>
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  );
}
