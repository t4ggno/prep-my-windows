import { useEffect, useRef } from "react";
import { LoaderCircle, Search, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import { cn } from "@/lib/utils";

export function ViewHeader({
  title,
  actions,
}: {
  title: string;
  actions?: React.ReactNode;
}) {
  return (
    <header className="view-header">
      <h1>{title}</h1>
      {actions && <div className="flex items-center gap-2">{actions}</div>}
    </header>
  );
}

export function SearchField({
  value,
  onChange,
  placeholder = "Search",
  className,
}: {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  className?: string;
}) {
  const input = useRef<HTMLInputElement>(null);

  useEffect(() => {
    function focusOnShortcut(event: KeyboardEvent) {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "f") {
        event.preventDefault();
        input.current?.focus();
        input.current?.select();
      }
    }
    window.addEventListener("keydown", focusOnShortcut);
    return () => window.removeEventListener("keydown", focusOnShortcut);
  }, []);

  return (
    <div className={cn("relative w-64", className)}>
      <Search className="pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-zinc-600" />
      <Input
        ref={input}
        value={value}
        onChange={(event) => onChange(event.target.value)}
        onKeyDown={(event) => {
          if (event.key === "Escape" && value) {
            event.stopPropagation();
            onChange("");
          }
        }}
        placeholder={placeholder}
        className="px-9"
      />
      {value && (
        <button
          className="absolute right-2 top-1/2 grid size-6 -translate-y-1/2 place-items-center rounded-md text-zinc-500 hover:bg-white/[0.06] hover:text-zinc-200"
          onClick={() => onChange("")}
          aria-label="Clear search"
        >
          <X className="size-3.5" />
        </button>
      )}
    </div>
  );
}

export function StateDot({ active }: { active: boolean }) {
  return (
    <span
      className={cn(
        "inline-block size-1.5 shrink-0 rounded-full",
        active ? "bg-emerald-400" : "bg-zinc-600",
      )}
    />
  );
}

export function EmptyState({ children }: { children: React.ReactNode }) {
  return <div className="empty-state">{children}</div>;
}

export function LoadingState() {
  return (
    <div className="empty-state">
      <LoaderCircle className="size-5 animate-spin text-zinc-500" />
    </div>
  );
}

export interface FilterOption<T extends string> {
  id: T;
  label: string;
  count?: number;
}

export function FilterPills<T extends string>({
  options,
  value,
  onChange,
}: {
  options: FilterOption<T>[];
  value: T;
  onChange: (value: T) => void;
}) {
  return (
    <div className="filter-pills" role="tablist">
      {options.map((option) => (
        <button
          key={option.id}
          role="tab"
          aria-selected={value === option.id}
          onClick={() => onChange(option.id)}
          className={cn(value === option.id && "active")}
        >
          {option.label}
          {option.count !== undefined && <span className="pill-count">{option.count}</span>}
        </button>
      ))}
    </div>
  );
}

export function SegmentedControl<T extends string>({
  options,
  value,
  onChange,
}: {
  options: FilterOption<T>[];
  value: T;
  onChange: (value: T) => void;
}) {
  return (
    <div className="segmented-control" role="tablist">
      {options.map((option) => (
        <button
          key={option.id}
          role="tab"
          aria-selected={value === option.id}
          className={cn(value === option.id && "active")}
          onClick={() => onChange(option.id)}
        >
          {option.label}
          {option.count !== undefined && <span className="pill-count">{option.count}</span>}
        </button>
      ))}
    </div>
  );
}

export function BulkToggle({
  items,
  pending,
  onChange,
}: {
  items: Array<{ enabled: boolean }>;
  pending: boolean;
  onChange: (enabled: boolean) => void;
}) {
  return (
    <div className="flex shrink-0 items-center gap-2">
      <Button
        size="sm"
        variant="secondary"
        disabled={pending || items.every((item) => item.enabled)}
        onClick={() => onChange(true)}
      >
        Turn all on
      </Button>
      <Button
        size="sm"
        variant="secondary"
        disabled={pending || items.every((item) => !item.enabled)}
        onClick={() => onChange(false)}
      >
        Turn all off
      </Button>
    </div>
  );
}

export function RuleSwitch({
  checked,
  pending,
  disabled,
  onCheckedChange,
  label,
}: {
  checked: boolean;
  pending: boolean;
  disabled?: boolean;
  onCheckedChange: (checked: boolean) => void;
  label: string;
}) {
  return (
    <Switch
      checked={checked}
      disabled={disabled || pending}
      data-pending={pending || undefined}
      onCheckedChange={onCheckedChange}
      aria-label={label}
      aria-busy={pending}
    />
  );
}
