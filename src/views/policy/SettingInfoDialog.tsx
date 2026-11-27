import { Markdown } from "@/components/Markdown";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog";
import { settingDoc, taskDoc } from "@/content/docs";
import { cn } from "@/lib/utils";
import type { PolicyItem, ScheduledTaskItem, TaskStatus } from "@/types";
import { policyIsMismatched, valueDiffersFromRecommendation } from "@/views/policy/policyFilters";

export type InfoTarget =
  | { kind: "policy"; item: PolicyItem }
  | { kind: "task"; item: ScheduledTaskItem };

export const taskStatusLabels: Record<TaskStatus, string> = {
  enabled: "Enabled",
  disabled: "Disabled",
  missing: "Not present",
  unknown: "Unknown",
};

interface Fact {
  label: string;
  value: string;
  tone?: "warning" | "accent";
}

function policyFacts(item: PolicyItem): Fact[] {
  if (!item.available) {
    return [
      { label: "Status", value: item.unavailableReason ?? "Unavailable", tone: "warning" },
      { label: "Recommended", value: item.recommended },
    ];
  }
  return [
    { label: "Current", value: item.current, tone: policyIsMismatched(item) ? "warning" : undefined },
    {
      label: "Wanted",
      value: item.wanted,
      tone: valueDiffersFromRecommendation(item) ? "accent" : undefined,
    },
    { label: "Recommended", value: item.recommended },
    { label: "Rule", value: item.enabled ? "Enforced" : "Not enforced" },
  ];
}

function taskFacts(task: ScheduledTaskItem): Fact[] {
  return [
    {
      label: "Task",
      value: taskStatusLabels[task.status],
      tone: task.status === "enabled" ? "warning" : undefined,
    },
    { label: "Wanted", value: "Disabled" },
    { label: "Rule", value: task.enabled ? "Enforced" : "Not enforced" },
  ];
}

export function SettingInfoDialog({
  target,
  onClose,
}: {
  target: InfoTarget | null;
  onClose: () => void;
}) {
  return (
    <Dialog open={target !== null} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="info-dialog" aria-describedby={undefined}>
        {target && <InfoContent target={target} />}
      </DialogContent>
    </Dialog>
  );
}

function InfoContent({ target }: { target: InfoTarget }) {
  const isPolicy = target.kind === "policy";
  const facts = isPolicy ? policyFacts(target.item) : taskFacts(target.item);
  const doc = isPolicy ? settingDoc(target.item.id) : taskDoc(target.item.id);
  const eyebrow = isPolicy
    ? `${target.item.category} · ${target.item.scope}`
    : "Scheduled task";

  return (
    <>
      <header className="info-header">
        <div className="info-eyebrow">{eyebrow}</div>
        <DialogTitle className="text-lg tracking-tight">{target.item.name}</DialogTitle>
      </header>
      <div className="info-body">
        <dl className="info-facts">
          {facts.map((fact) => (
            <div key={fact.label} className={cn("info-fact", fact.tone)}>
              <dt>{fact.label}</dt>
              <dd>{fact.value}</dd>
            </div>
          ))}
        </dl>
        {doc && <Markdown>{doc}</Markdown>}
        {isPolicy && target.item.registry && (
          <div className="info-location">
            <span>Registry</span>
            <code>{target.item.registry.key}</code>
            <code>{target.item.registry.value}</code>
          </div>
        )}
        {!isPolicy && (
          <div className="info-location">
            <span>Task path</span>
            <code>{target.item.path}</code>
          </div>
        )}
      </div>
    </>
  );
}
