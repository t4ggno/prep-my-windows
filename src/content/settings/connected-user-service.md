**Connected User Experiences and Telemetry** (`DiagTrack`) is the main Windows service that uploads diagnostic data.

### Why it matters
- Disabling it stops the telemetry uploads at the source, regardless of other settings.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Automatic** | `2` | Windows default. Starts with Windows. |
| **Manual** | `3` | Starts when requested. |
| **Disabled** | `4` | Never starts. |

> **Good to know:** Windows Insider builds and some Windows Update diagnostics need this service.
