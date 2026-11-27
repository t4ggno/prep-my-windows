Decides whether the PC stays connected to the network during **Modern Standby** (sleep) while it is plugged in.

### Why it matters
- Without network access in standby, Windows cannot download updates or wake for background tasks while you are away.
- Sleep becomes quieter and more predictable.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **On** | `1` | The network stays connected during standby. |
| **Off** | `0` | The network disconnects during standby. |

> **Good to know:** Only affects PCs with Modern Standby (S0). Email and messages sync again right after waking.
