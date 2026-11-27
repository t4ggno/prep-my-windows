Decides whether Windows may **restart automatically** for updates while someone is signed in.

### Why it matters
- Prevents losing unsaved work or long-running jobs to an overnight update restart.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **On** | `0` | Windows default. Windows may restart while you are signed in. |
| **Off** | `1` | Windows waits until you restart yourself. |

> **Good to know:** Updates are installed but only finish after your next restart, so restart regularly.
