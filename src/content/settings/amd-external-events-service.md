The **AMD External Events Utility** (`atiesrxx.exe` with its client `atieclxx.exe`) passes hotkeys and display events to AMD Software: Adrenalin Edition.

### Why it matters
- Some driver releases leak memory in `atieclxx.exe`, reserving many gigabytes of memory over days.
- Driver updates set the service back to *Automatic*; this rule disables it again and stops both processes.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Automatic** | `2` | Starts with Windows. |
| **Manual** | `3` | Starts only when an app needs it. |
| **Disabled** | `4` | Never starts. |

> **Good to know:** Adrenalin hotkeys such as the overlay (Alt+R) and recording shortcuts stop working. Hot-plugging monitors, display mode switches and FreeSync can behave differently, especially with external GPUs.
