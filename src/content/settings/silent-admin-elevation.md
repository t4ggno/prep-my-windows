Controls what happens when an app run by an **administrator** asks for elevated rights — the User Account Control (UAC) prompt.

### Why it matters
- With **Never ask**, admin accounts elevate silently: no dimmed screen, no extra click for installers, tools and settings pages.
- Standard (non-admin) accounts are not affected; they still need credentials.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Never ask** | `0` | Elevates immediately without any prompt. |
| **Ask for credentials** | `1` | Asks for a user name and password on the secure (dimmed) desktop. |
| **Always ask** | `2` | Asks for consent on the secure desktop, also for Windows' own tools. |
| **Ask for credentials without dimming** | `3` | Asks for a password without switching to the secure desktop. |
| **Always ask without dimming** | `4` | Asks for consent without switching to the secure desktop. |
| **Ask for non-Windows apps** | `5` | Windows default. Asks only for apps that are not part of Windows. |

> **Good to know:** UAC prompts are a last line of defence against malware that tries to elevate itself. Pick **Ask for non-Windows apps** if other people or untrusted software use this account.
