The **diagnostic data choice** stored by the Settings app, separate from the group policy.

### Why it matters
- Keeps the Settings toggle consistent with the diagnostic data policy, so Windows does not switch back to sending optional data.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Off** | `0` | Security level (honoured only on Enterprise and Education). |
| **Required** | `1` | Required diagnostic data only. |
| **Enhanced** | `2` | Windows 10 level between Required and Optional. |
| **Optional** | `3` | Full diagnostic data. |
