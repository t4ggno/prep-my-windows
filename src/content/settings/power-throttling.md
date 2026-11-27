**Power throttling** lowers the CPU priority and clock speed of apps that run in the background to save power.

### Why it matters
- Background work such as video encoding, downloads or game launchers can become slower while their window is not in front.
- Turning throttling off gives all apps full performance at the cost of battery life.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **On** | `0` | Windows default. Background apps can be throttled. |
| **Off** | `1` | Power throttling is disabled for all apps. |

> **Good to know:** This rule is optional. It mainly makes sense on desktop PCs.
