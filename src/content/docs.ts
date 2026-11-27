const settingDocs = import.meta.glob<string>("./settings/*.md", {
  query: "?raw",
  import: "default",
  eager: true,
});

const taskDocs = import.meta.glob<string>("./tasks/*.md", {
  query: "?raw",
  import: "default",
  eager: true,
});

export function settingDoc(id: string): string | undefined {
  return settingDocs[`./settings/${id}.md`];
}

export function taskDoc(id: string): string | undefined {
  return taskDocs[`./tasks/${id}.md`];
}
