import { ActiveHoursPicker } from "@/components/ActiveHoursPicker";
import { Select } from "@/components/ui/input";
import type { ActiveHours, PolicyItem } from "@/types";

export function PolicyValueEditor({
  item,
  activeHours,
  choicePending,
  activeHoursPending,
  onChoose,
  onChangeActiveHours,
}: {
  item: PolicyItem;
  activeHours: ActiveHours;
  choicePending: boolean;
  activeHoursPending: boolean;
  onChoose: (choice: string) => void;
  onChangeActiveHours: (hours: ActiveHours) => void;
}) {
  if (!item.available) return <code className="truncate">—</code>;

  if (item.editor === "activeHoursStart" || item.editor === "activeHoursEnd") {
    return (
      <ActiveHoursPicker
        className="h-7 text-xs"
        hours={activeHours}
        edge={item.editor === "activeHoursStart" ? "start" : "end"}
        disabled={activeHoursPending}
        onChange={onChangeActiveHours}
      />
    );
  }

  if (item.choices.length < 2) return <code className="truncate">{item.wanted}</code>;

  return (
    <Select
      className="h-7 rounded-md px-2 text-xs"
      value={item.choice ?? ""}
      disabled={choicePending}
      aria-label={`${item.name} value`}
      onChange={(event) => onChoose(event.target.value)}
    >
      {item.choices.map((option) => (
        <option key={option.id} value={option.id}>
          {option.label}
        </option>
      ))}
    </Select>
  );
}
