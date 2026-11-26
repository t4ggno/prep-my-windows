import { createHash, createPublicKey, timingSafeEqual, verify } from "node:crypto";
import { execFileSync } from "node:child_process";
import { createReadStream } from "node:fs";
import { readFile, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";

const REPOSITORY = "t4ggno/prep-my-windows";
const PACKAGE = "prep-my-windows-windows-x64-setup.exe";
const TARGET = "windows-x86_64-nsis";
const MANIFEST = "latest.json";
const CONFIGURATION = resolve("src-tauri/tauri.conf.json");

function decodeBase64(value) {
  const encoded = value.trim();
  if (!/^[A-Za-z0-9+/]+={0,2}$/.test(encoded)) throw new Error("Invalid signature encoding");
  return Buffer.from(encoded, "base64");
}

function signedMetadata(signatureLines) {
  const fields = signatureLines[2].slice("trusted comment: ".length).split("\t");
  const versions = fields.filter((field) => field.startsWith("version:"));
  const names = fields.filter((field) => field.startsWith("file:"));
  if (versions.length !== 1 || names.length !== 1)
    throw new Error("The signature must identify one version and file name");
  return { version: versions[0].slice("version:".length), fileName: names[0].slice("file:".length) };
}

async function blake2b(path) {
  const digest = createHash("blake2b512");
  for await (const chunk of createReadStream(path)) digest.update(chunk);
  return digest.digest();
}

async function verifySignature(directory, publicKey, version) {
  const artifact = join(directory, PACKAGE);
  const encodedSignature = await readFile(`${artifact}.sig`, "utf8");
  const publicLines = decodeBase64(publicKey).toString("utf8").trim().split(/\r?\n/);
  const signatureLines = decodeBase64(encodedSignature).toString("utf8").trim().split(/\r?\n/);
  if (publicLines.length !== 2 || signatureLines.length !== 4)
    throw new Error("Invalid signature format");
  const publicPacket = decodeBase64(publicLines[1]);
  const signaturePacket = decodeBase64(signatureLines[1]);
  if (
    publicPacket.length !== 42 ||
    publicPacket.subarray(0, 2).toString() !== "Ed" ||
    signaturePacket.length !== 74 ||
    signaturePacket.subarray(0, 2).toString() !== "ED" ||
    !timingSafeEqual(publicPacket.subarray(2, 10), signaturePacket.subarray(2, 10)) ||
    !signatureLines[2].startsWith("trusted comment: ")
  )
    throw new Error("The signature does not match the committed public key");
  const key = createPublicKey({
    key: Buffer.concat([Buffer.from("302a300506032b6570032100", "hex"), publicPacket.subarray(10)]),
    format: "der",
    type: "spki",
  });
  const signature = signaturePacket.subarray(10);
  const comment = signatureLines[2].slice("trusted comment: ".length);
  const globalSignature = decodeBase64(signatureLines[3]);
  if (
    globalSignature.length !== 64 ||
    !verify(null, await blake2b(artifact), key, signature) ||
    !verify(null, Buffer.concat([signature, Buffer.from(comment)]), key, globalSignature)
  )
    throw new Error(`Signature verification failed: ${PACKAGE}`);
  const signed = signedMetadata(signatureLines);
  if (signed.version !== version || signed.fileName !== PACKAGE)
    throw new Error(`The signature is not bound to ${version} and ${PACKAGE}`);
  return encodedSignature.trim();
}

function sign(directory, version) {
  if (!process.env.TAURI_SIGNING_PRIVATE_KEY)
    throw new Error("TAURI_SIGNING_PRIVATE_KEY is required to sign updates");
  const signer = createRequire(resolve("package.json")).resolve("@tauri-apps/cli/tauri.js");
  execFileSync(
    process.execPath,
    [signer, "signer", "sign", "--app-version", version, resolve(directory, PACKAGE)],
    { stdio: ["ignore", "ignore", "inherit"], windowsHide: true },
  );
}

async function buildManifest(directory, version, publicKey, releasePath) {
  const signature = await verifySignature(directory, publicKey, version);
  const release = JSON.parse(await readFile(releasePath, "utf8"));
  const manifest = {
    version,
    notes: typeof release.body === "string" ? release.body : "",
    pub_date: new Date().toISOString(),
    platforms: {
      [TARGET]: {
        url: `https://github.com/${REPOSITORY}/releases/download/v${version}/${PACKAGE}`,
        signature,
      },
    },
  };
  await writeFile(join(directory, MANIFEST), `${JSON.stringify(manifest, null, 2)}\n`);
}

async function verifyAssets(releasePath) {
  const names = JSON.parse(await readFile(releasePath, "utf8")).assets.map((asset) => asset.name);
  const expected = [PACKAGE, `${PACKAGE}.sig`, MANIFEST];
  const unexpected = names.filter((name) => !expected.includes(name));
  const missing = expected.filter((name) => names.filter((candidate) => candidate === name).length !== 1);
  if (missing.length || unexpected.length)
    throw new Error(
      `Release assets are incomplete. Missing or duplicated: ${missing.join(", ") || "none"}. Unexpected: ${unexpected.join(", ") || "none"}`,
    );
}

async function main() {
  const [command, path, releasePath] = process.argv.slice(2);
  const configuration = JSON.parse(await readFile(CONFIGURATION, "utf8"));
  const version = configuration.version;
  if (!/^\d+\.\d+\.\d+$/.test(version)) throw new Error("Updater releases must use a stable version");
  const publicKey = configuration.plugins.updater.pubkey;
  switch (command) {
    case "sign":
      sign(path, version);
      await verifySignature(path, publicKey, version);
      return;
    case "build":
      await buildManifest(path, version, publicKey, releasePath);
      return;
    case "verify":
      await verifyAssets(path);
      return;
    default:
      throw new Error("Usage: update-manifest.mjs <sign|build> <directory> [release-json] | verify <release-json>");
  }
}

main().catch((error) => {
  console.error(error.message);
  process.exitCode = 1;
});
