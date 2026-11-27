**Hardware-accelerated GPU scheduling** (HAGS) lets the graphics card manage its own memory and work queue.

### Why it matters
- Can lower latency and CPU load in games, and is required for NVIDIA DLSS Frame Generation.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **On** | `2` | The GPU schedules its own work. |
| **Off** | `1` | Windows schedules GPU work. |

> **Good to know:** Takes effect after a restart and requires a supported GPU and driver. Switch it off if you see stutter in specific games.
