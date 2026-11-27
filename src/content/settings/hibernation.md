Keeps **hibernation** available: the PC saves your open session to disk and turns off completely, using no power.

### Why it matters
- With fast startup off, hibernation is the way to resume a session after a full power-off.
- A full hibernation file is needed; the reduced file only supports fast startup.

### Values
| State | Meaning |
| --- | --- |
| **Enabled** | Hibernation is available with a full hibernation file. |
| **Fast startup only** | Only the reduced file exists; hibernation itself is not possible. |
| **Disabled** | No hibernation file exists. |

> **Good to know:** The hibernation file (`hiberfil.sys`) takes several gigabytes on the system drive.
