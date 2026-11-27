**Fast startup** saves the Windows kernel to disk at shutdown so the next start is faster — a partial hibernation.

### Why it matters
- With fast startup, *Shut down* is not a full restart: driver problems, pending updates and uptime carry over.
- It can lock drives for dual-boot systems and cause wake or USB issues.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **On** | `1` | Windows default. Shutdown keeps a saved kernel session. |
| **Off** | `0` | Every shutdown is a clean, full shutdown. |

> **Good to know:** Starting may take a few seconds longer on older hard drives. Hibernation stays available separately.
