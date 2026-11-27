The **Multimedia Class Scheduler** reserves a share of CPU time for background tasks while audio or games run (`SystemResponsiveness`).

### Why it matters
- A lower reservation gives games and audio apps more CPU time.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **10%** | `10` | Reserves 10% for background tasks — the lowest effective value. |
| **20%** | `20` | Windows default. Reserves 20% for background tasks. |

> **Good to know:** This rule is optional. Windows treats values below 10 as 10, so `0` brings no extra benefit.
