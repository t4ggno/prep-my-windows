**AutoRun** starts programs automatically from inserted media such as USB sticks or discs (via `autorun.inf`).

### Why it matters
- AutoRun was a common way for malware to spread through USB drives.
- Turning it off for all drive types closes that door completely.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Off on all drives** | `255` | AutoRun never runs, on any drive type. |
| **Windows default** | Not set | AutoRun is off only for unknown and network drives (`0x91`). |

> **Good to know:** This is different from **AutoPlay**, which only asks what to do with new media.
