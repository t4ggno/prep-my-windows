# Prep My Windows

Prep My Windows applies a personal configuration to Windows 11 and keeps the selected rules in place while it is running. It can change Windows and Microsoft Edge settings, remove built-in and Store apps, prevent software from starting automatically, block selected Microsoft connections, disable background tasks, and stop chosen programs.

## Before you start

> [!WARNING]
> **Selections take effect immediately.** On first launch, the built-in profile is already enabled and enforcement starts automatically. Rule switches and action buttons do not wait for a separate **Save**, **Apply**, or confirmation step. Some work may finish during the next enforcement cycle; **Enforce now** runs all enabled rules at once. The interval, active-hours, and **Start with Windows** controls are the only exception and require **Save settings**.

The default profile is broad: all built-in rules are selected, including app-removal and process rules for Microsoft and third-party software. Use Prep My Windows only on a Windows installation you intend to configure this way.

- App removal affects all users on the PC and prevents removed packages from being added for new users. Turning a rule off does not reinstall an app.
- Process rules force matching programs to close as soon as the app detects them and can discard unsaved work.
- Blocking an autostart entry can remove it or disable its service or scheduled task. Removing the rule later does not recreate or re-enable the original entry.
- The default Windows policy changes system-wide behavior, including whether Windows asks for approval before administrator-level changes, diagnostics, cloud features, Windows Update, and Edge. Network blocks may also prevent related Microsoft services from connecting.

Turning off a rule stops Prep My Windows from enforcing it again, but it does not necessarily restore the previous Windows setting. Quitting or uninstalling the app also does not undo changes already made.

## Requirements

- A 64-bit PC running Windows 11
- A Windows account with administrator access

## Install and open

1. Download the Prep My Windows installer (the `.exe` file ending in `_x64-setup.exe`).
2. Open the installer and follow the prompts. It installs for your Windows account.
3. Open **Prep My Windows** from the Start menu or its desktop shortcut.
4. Select **Yes** when Windows requests administrator access. The app cannot run without it.

The installer is not digitally signed, so Microsoft Defender SmartScreen may warn you about it. Continue only if you obtained the file from a source you trust.

## Use the app

| Area | What it does |
| --- | --- |
| **Overview** | Shows whether the policy is running, the last enforcement time, rule totals, and recent activity. |
| **Windows policy** | Lets you search and filter Windows and Edge settings. **Current** shows the detected value and **Wanted** shows the value the app will enforce. Unavailable settings are disabled and show why they are unsupported. |
| **Apps** | Enables removal rules and lists installed packages. In **Installed**, **Block** adds a package to the removal rules. |
| **Autostart** | Lists programs, services, and tasks that start automatically. **Block** disables the selected entry and keeps it blocked. |
| **Process rules** | Stops matching programs whenever they run. Add a rule from a running process or by choosing an `.exe` file. |
| **Network** | Blocks or unblocks the listed Microsoft endpoints. |
| **Activity** | Shows changes and errors from the current session. |
| **Settings** | Changes enforcement intervals, Windows Update active hours, startup behavior, and profile import or export. |

For switches, **on** means Prep My Windows will enforce the displayed action: apply the wanted setting, remove the app, disable the task, block the endpoint, or stop the process. Use **Enforce now** to check and apply every enabled rule immediately.

## Settings and profiles

On the **Settings** page, enter active hours as whole hours from `0` to `23`. The period from start to end must be between 1 and 18 hours. Select **Save settings** after changing intervals, active hours, or **Start with Windows**.

- **Export** saves a copy of the current profile.
- **Import** replaces the current profile with the selected profile.
- **Reset profile** immediately restores the app's built-in profile, with its rules enabled. It does not restore Windows defaults and does not ask for confirmation.

## Keep running or quit

Closing the window hides Prep My Windows in the system tray; enforcement continues in the background. Select the tray icon to reopen it, or right-click it for **Open**, **Enforce now**, and **Quit**.

**Start with Windows** is enabled by default. To prevent the app from starting at your next sign-in, turn that setting off and select **Save settings**. To stop enforcement for the current session, choose **Quit** from the tray menu.
