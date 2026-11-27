use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ChangeSection {
    Setting,
    App,
    Network,
    ScheduledTask,
    Autostart,
}

impl ChangeSection {
    fn key_prefix(self) -> &'static str {
        match self {
            Self::Setting => "setting",
            Self::App => "app",
            Self::Network => "network",
            Self::ScheduledTask => "task",
            Self::Autostart => "autostart",
        }
    }

    pub fn key(self, id: &str) -> String {
        format!("{}:{}", self.key_prefix(), id.to_ascii_lowercase())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppliedChange {
    pub key: String,
    pub section: ChangeSection,
    pub name: String,
    pub found: String,
    pub wanted: String,
}

impl AppliedChange {
    pub fn new(
        section: ChangeSection,
        id: &str,
        name: impl Into<String>,
        found: impl Into<String>,
        wanted: impl Into<String>,
    ) -> Self {
        Self {
            key: section.key(id),
            section,
            name: name.into(),
            found: found.into(),
            wanted: wanted.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedChange {
    pub change: AppliedChange,
    pub reverts: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeSummary {
    pub message: String,
    pub details: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct HistoryEntry {
    name: String,
    section: ChangeSection,
    applied: Option<String>,
    reverts: u64,
    last_reverted_at: Option<String>,
    last_found: Option<String>,
    last_wanted: Option<String>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReappliedChange {
    pub key: String,
    pub name: String,
    pub section: ChangeSection,
    pub reverts: u64,
    pub last_reverted_at: String,
    pub last_found: String,
    pub wanted: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHistory {
    entries: BTreeMap<String, HistoryEntry>,
}

impl ChangeHistory {
    pub fn record(&mut self, changes: Vec<AppliedChange>, timestamp: &str) -> Vec<RecordedChange> {
        changes
            .into_iter()
            .map(|change| {
                let entry =
                    self.entries
                        .entry(change.key.clone())
                        .or_insert_with(|| HistoryEntry {
                            name: change.name.clone(),
                            section: change.section,
                            applied: None,
                            reverts: 0,
                            last_reverted_at: None,
                            last_found: None,
                            last_wanted: None,
                        });
                entry.name.clone_from(&change.name);
                entry.section = change.section;
                let reverted = entry.applied.as_deref() == Some(change.wanted.as_str());
                if reverted {
                    entry.reverts += 1;
                    entry.last_reverted_at = Some(timestamp.to_owned());
                    entry.last_found = Some(change.found.clone());
                    entry.last_wanted = Some(change.wanted.clone());
                } else {
                    entry.applied = Some(change.wanted.clone());
                }
                RecordedChange {
                    reverts: reverted.then_some(entry.reverts),
                    change,
                }
            })
            .collect()
    }

    pub fn release<'a>(&mut self, keys: impl IntoIterator<Item = &'a str>) -> bool {
        let mut changed = false;
        for key in keys {
            if let Some(entry) = self.entries.get_mut(key) {
                changed |= entry.applied.take().is_some();
            }
        }
        changed
    }

    pub fn release_all(&mut self) -> bool {
        let mut changed = false;
        for entry in self.entries.values_mut() {
            changed |= entry.applied.take().is_some();
        }
        changed
    }

    pub fn total_reverts(&self) -> u64 {
        self.entries.values().map(|entry| entry.reverts).sum()
    }

    pub fn reapplied(&self) -> Vec<ReappliedChange> {
        let mut changes = self
            .entries
            .iter()
            .filter_map(|(key, entry)| {
                Some(ReappliedChange {
                    key: key.clone(),
                    name: entry.name.clone(),
                    section: entry.section,
                    reverts: entry.reverts,
                    last_reverted_at: entry.last_reverted_at.clone()?,
                    last_found: entry.last_found.clone()?,
                    wanted: entry.last_wanted.clone()?,
                })
            })
            .collect::<Vec<_>>();
        changes.sort_by(|left, right| right.last_reverted_at.cmp(&left.last_reverted_at));
        changes
    }
}

fn times(count: u64) -> String {
    if count == 1 {
        "once".to_owned()
    } else {
        format!("{count} times")
    }
}

fn summary(verb: &str, changes: &[&RecordedChange]) -> Option<ChangeSummary> {
    let message = match changes {
        [] => return None,
        [single] => format!("{verb} {}", single.change.name),
        changes => format!("{verb} {} changes", changes.len()),
    };
    let details = changes
        .iter()
        .map(|record| {
            let change = &record.change;
            let line = format!("{}: {} → {}", change.name, change.found, change.wanted);
            match record.reverts {
                Some(count) => format!("{line} · changed back {}", times(count)),
                None => line,
            }
        })
        .collect();
    Some(ChangeSummary { message, details })
}

pub fn summaries(records: &[RecordedChange]) -> Vec<ChangeSummary> {
    let (reapplied, applied): (Vec<_>, Vec<_>) =
        records.iter().partition(|record| record.reverts.is_some());
    [
        summary("Reapplied", &reapplied),
        summary("Applied", &applied),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::{AppliedChange, ChangeHistory, ChangeSection, summaries};

    fn change(found: &str, wanted: &str) -> AppliedChange {
        AppliedChange::new(
            ChangeSection::Setting,
            "Feature-Updates",
            "Early feature updates",
            found,
            wanted,
        )
    }

    #[test]
    fn counts_only_changes_made_after_the_first_application() {
        let mut history = ChangeHistory::default();

        let first = history.record(vec![change("1", "0")], "2026-11-27T10:00:00Z");
        let second = history.record(vec![change("1", "0")], "2026-11-27T11:00:00Z");
        let third = history.record(vec![change("Not set", "0")], "2026-11-27T12:00:00Z");

        assert_eq!(first[0].reverts, None);
        assert_eq!(second[0].reverts, Some(1));
        assert_eq!(third[0].reverts, Some(2));
        assert_eq!(history.total_reverts(), 2);
        let reapplied = history.reapplied();
        assert_eq!(reapplied.len(), 1);
        assert_eq!(reapplied[0].key, "setting:feature-updates");
        assert_eq!(reapplied[0].last_found, "Not set");
        assert_eq!(reapplied[0].last_reverted_at, "2026-11-27T12:00:00Z");
    }

    #[test]
    fn a_new_wanted_value_is_a_fresh_application() {
        let mut history = ChangeHistory::default();
        history.record(vec![change("1", "0")], "t1");

        let changed_choice = history.record(vec![change("0", "2")], "t2");

        assert_eq!(changed_choice[0].reverts, None);
        assert_eq!(history.total_reverts(), 0);
    }

    #[test]
    fn released_changes_keep_their_count_but_restart_tracking() {
        let mut history = ChangeHistory::default();
        history.record(vec![change("1", "0")], "t1");
        history.record(vec![change("1", "0")], "t2");

        assert!(history.release(["setting:feature-updates"]));
        let reapplied = history.record(vec![change("1", "0")], "t3");

        assert_eq!(reapplied[0].reverts, None);
        assert_eq!(history.total_reverts(), 1);
        assert_eq!(history.reapplied()[0].reverts, 1);
        assert!(!history.release(["setting:missing"]));
    }

    #[test]
    fn summarizes_reapplied_and_applied_changes_separately() {
        let mut history = ChangeHistory::default();
        history.record(vec![change("1", "0")], "t1");
        let records = history.record(
            vec![
                change("1", "0"),
                AppliedChange::new(
                    ChangeSection::App,
                    "Microsoft.BingNews",
                    "News",
                    "Installed",
                    "Removed",
                ),
            ],
            "t2",
        );

        let summaries = summaries(&records);

        assert_eq!(summaries.len(), 2);
        assert_eq!(summaries[0].message, "Reapplied Early feature updates");
        assert_eq!(
            summaries[0].details,
            ["Early feature updates: 1 → 0 · changed back once"]
        );
        assert_eq!(summaries[1].message, "Applied News");
        assert_eq!(summaries[1].details, ["News: Installed → Removed"]);
    }
}
