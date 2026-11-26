# Prep My Windows

Prep My Windows applies a personal configuration to Windows 11 and keeps the selected rules in place while it is running. It can change Windows and Microsoft Edge settings, remove built-in and Store apps, prevent software from starting automatically, block selected Microsoft connections, disable background tasks, and stop chosen programs.

## Before you start

> [!WARNING]
> **Enforcement starts off.** On first launch, the built-in profile is selected but nothing is applied, so you can review and adjust the rules first. Select **Enable** in the top bar and confirm to apply every enabled rule at once; from then on, selections take effect immediately. Rule switches and most action buttons do not wait for a separate **Save** or **Apply** step. **Disable** stops all enforcement again without restoring changes already made. The active-hours and **Start with Windows** controls require **Save settings**.

The default profile is broad: nearly all built-in rules are selected, including app-removal and process rules for Microsoft and third-party software. A few rules that change personal preferences or remove protections start off, such as taskbar alignment, File Explorer layout, camera and microphone blocks, driver-update exclusion, and automatic device encryption. Use Prep My Windows only on a Windows installation you intend to configure this way.

- App removal affects all users on the PC and prevents removed packages from being added for new users. Turning a rule off does not reinstall an app.
- Process rules force matching programs to close as soon as the app detects them and can discard unsaved work. The notification can allow the stopped app once or disable its rule permanently.
- Blocking an autostart entry can remove it or disable its service or scheduled task. Removing the rule later does not recreate or re-enable the original entry.
- The default Windows policy changes system-wide behavior, including whether Windows asks for approval before administrator-level changes, diagnostics, app permissions, Copilot and other AI features, search, cloud features, fast startup, network use in standby, game capture, mouse acceleration, GPU scheduling, the File Explorer context menu, AutoPlay and AutoRun, Remote Assistance, Windows Update restarts, Defender sample submission, software installed by device firmware, several background services, Windows User Choice Protection (takes effect after a restart), and Edge. Some changes take effect only after you sign out or restart. Network blocks may also prevent related Microsoft services from connecting.

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
| **Windows policy** | Lets you search and filter Windows and Edge settings by category or by whether they are applied. **Current** shows the detected value and **Wanted** shows the value the app will enforce. Settings with more than one useful value, such as diagnostic data, service startup, app permissions, location, administrator prompts, Defender sample submission, update sharing, and taskbar search, let you choose that value from a list. The first option is the built-in default. Unavailable settings, including services that are not installed, are disabled and show why. |
| **Apps** | Enables removal rules and lists installed packages. In **Installed**, **Remove** removes a package for all users and adds it to the removal rules. |
| **Autostart** | Lists programs, services, and tasks that start automatically. **Block** disables the selected entry and keeps it blocked. |
| **Process rules** | Stops matching programs whenever they run. Add a rule from a running process or by choosing an `.exe` file. **Notify** controls stop notifications without changing the rule. |
| **Network** | Blocks or unblocks the listed Microsoft endpoints. |
| **Activity** | Shows changes and errors from the current session. Filter by errors or search messages. |
| **Settings** | Changes Windows Update active hours, startup behavior, and profile import or export. |

For switches, **on** means Prep My Windows will enforce the displayed action: apply the wanted setting, remove the app, disable the task, block the endpoint, or stop the process. While enforcement is enabled, use **Enforce now** to check and apply every enabled rule immediately. **Turn all on** and **Turn all off** change every rule currently shown on **Windows policy**, **Apps**, and **Network**, so you can combine them with search and filters. Press **Ctrl+F** to search the current page.

## Settings and profiles

On the **Settings** page, choose the start and end of Windows Update active hours. The period from start to end must be between 1 and 18 hours. Select **Save settings** after changing active hours or **Start with Windows**.

- **Export** saves a copy of the current profile.
- **Import** replaces the current profile with the selected profile. It keeps the current **Enable**/**Disable** state.
- **Reset profile** restores the app's built-in profile and its default rule selection after you confirm. It keeps the current **Enable**/**Disable** state and does not restore Windows defaults.

## Keep running or quit

Closing the window hides Prep My Windows in the system tray; enforcement continues in the background. Select the tray icon to reopen it, or right-click it for **Open**, **Enforce now**, and **Quit**.

**Start with Windows** is enabled by default. To prevent the app from starting at your next sign-in, turn that setting off and select **Save settings**. To stop enforcement for the current session, choose **Quit** from the tray menu; to stop it permanently, select **Disable**.
