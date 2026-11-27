The **network connectivity probe** (NCSI) contacts a Microsoft server to check whether the internet is reachable.

### Why it matters
- The probe sends regular requests to `msftconnecttest.com`.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **On** | `0` | Windows default. Windows checks connectivity online. |
| **Off** | `1` | No active probes are sent. |

> **Good to know:** This rule is optional. Without the probe, Windows may show "No internet" although you are online, and captive portal login pages (hotels, trains) may not open automatically.
