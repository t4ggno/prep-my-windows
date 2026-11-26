const RELEASE_REPOSITORY = "t4ggno/prep-my-windows";
const UPDATE_PACKAGE = "prep-my-windows-windows-x64-setup.exe";

export function releaseNotesUrl(version: string) {
  return `https://github.com/${RELEASE_REPOSITORY}/releases/tag/v${version}`;
}

function parseSignedArtifact(encoded: string): { version: string; fileName: string } {
  let lines: string[];
  try {
    lines = atob(encoded.trim()).trim().split(/\r?\n/);
  } catch {
    throw new Error("Invalid update signature encoding");
  }
  if (lines.length !== 4 || !lines[2]?.startsWith("trusted comment: ")) {
    throw new Error("Invalid update signature format");
  }
  const fields = lines[2].slice("trusted comment: ".length).split("\t");
  const versions = fields.filter((field) => field.startsWith("version:"));
  const names = fields.filter((field) => field.startsWith("file:"));
  if (versions.length !== 1 || names.length !== 1) {
    throw new Error("The update signature must identify one version and file name");
  }
  return { version: versions[0].slice("version:".length), fileName: names[0].slice("file:".length) };
}

export function validateRelease(value: unknown) {
  if (!value || typeof value !== "object") throw new Error("Invalid update manifest");
  const manifest = value as Record<string, unknown>;
  const version = manifest.version;
  if (
    typeof version !== "string" ||
    !/^\d+\.\d+\.\d+$/.test(version) ||
    !manifest.platforms ||
    typeof manifest.platforms !== "object"
  ) {
    throw new Error("The update manifest must contain a stable release version and platforms");
  }
  const expectedUrl = `https://github.com/${RELEASE_REPOSITORY}/releases/download/v${version}/${UPDATE_PACKAGE}`;
  for (const [target, item] of Object.entries(manifest.platforms)) {
    const artifact = item as Record<string, unknown> | null;
    if (
      !artifact ||
      typeof artifact !== "object" ||
      artifact.url !== expectedUrl ||
      typeof artifact.signature !== "string"
    ) {
      throw new Error(`Invalid update package: ${target}`);
    }
    const signed = parseSignedArtifact(artifact.signature);
    if (signed.version !== version || signed.fileName !== UPDATE_PACKAGE) {
      throw new Error(`The update signature does not match its version and file: ${target}`);
    }
  }
}
