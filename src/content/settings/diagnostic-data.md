Sets the **diagnostic data** level that Windows sends to Microsoft, enforced as a policy.

### Why it matters
- *Optional* data includes browsing, app usage and inking details; *Required* still reports device and reliability data.
- The policy wins over the choice in *Settings → Privacy & security → Diagnostics & feedback*.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Off** | `0` | Security level. Only Enterprise, Education and Server editions honour it; other editions send *Required* data. |
| **Required** | `1` | Minimum data needed to keep Windows secure and up to date. |
| **Enhanced** | `2` | Windows 10 level between Required and Optional; Windows 11 treats it like Optional. |
| **Optional** | `3` | Full diagnostic data, including app and browsing activity. |

> **Good to know:** Combine it with the **Connected User Experiences service** rule to stop the telemetry service itself.
