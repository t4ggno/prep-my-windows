import { HourPicker } from "@/components/HourPicker";
import {
  isValidActiveHoursEnd,
  isWithinActiveHours,
  withActiveHoursStart,
} from "@/lib/activeHours";
import type { ActiveHours } from "@/types";

export function ActiveHoursPicker({
  hours,
  edge,
  disabled,
  className,
  onChange,
}: {
  hours: ActiveHours;
  edge: "start" | "end";
  disabled?: boolean;
  className?: string;
  onChange: (hours: ActiveHours) => void;
}) {
  const isStart = edge === "start";
  return (
    <HourPicker
      className={className}
      value={isStart ? hours.activeHoursStart : hours.activeHoursEnd}
      label={isStart ? "Active hours start" : "Active hours end"}
      disabled={disabled}
      isAllowed={isStart ? undefined : (hour) => isValidActiveHoursEnd(hours, hour)}
      isHighlighted={(hour) => isWithinActiveHours(hours, hour)}
      onChange={(hour) =>
        onChange(isStart ? withActiveHoursStart(hours, hour) : { ...hours, activeHoursEnd: hour })
      }
    />
  );
}
