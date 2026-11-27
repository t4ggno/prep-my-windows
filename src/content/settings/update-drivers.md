Decides whether Windows Update installs **drivers** together with quality updates.

### Why it matters
- Windows Update sometimes replaces a newer vendor driver (GPU, audio, chipset) with an older or generic one.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **On** | `0` | Windows default. Drivers are delivered by Windows Update. |
| **Off** | `1` | Drivers are excluded from Windows Update. |

> **Good to know:** This rule is optional. When it is on, install driver updates from your PC or GPU manufacturer yourself.
