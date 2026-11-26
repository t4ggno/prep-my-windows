# Releasing

A push to `main` that changes the version in `src-tauri/tauri.conf.json` publishes release `v<version>`. The versions in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json` must match. The workflow can also be started manually; it fails if the version's release is already published.

The workflow:

1. Creates a draft release with generated notes, or reuses an existing draft.
2. Builds the NSIS installer and copies it to `prep-my-windows-windows-x64-setup.exe`.
3. Signs the installer with `tauri signer sign --app-version`, which binds the signature to the version and file name, and verifies the signature against the public key in `src-tauri/tauri.conf.json`.
4. Uploads the installer and its signature, then generates and uploads `latest.json`.
5. Checks that the release contains exactly these three files, then publishes it as the latest release.

If a step fails, a draft created by that run is deleted.

Installed apps read `https://github.com/t4ggno/prep-my-windows/releases/latest/download/latest.json`. They accept an update only if the version is newer, the signature matches the built-in public key (`requireSignedVersion` is enabled), and the download URL and signed file name match the release.

## Signing key

GitHub Actions reads the repository secrets `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`. The encrypted private key, its password, and the public key are stored outside this checkout in `%LOCALAPPDATA%\prep-my-windows\release-signing`, which only the releasing user and SYSTEM can access. Back up that directory securely. If the key is lost or replaced, installed apps reject every new update until they are reinstalled manually with a release that contains the new public key.
