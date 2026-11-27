The **Parental Controls** service (`WpcMonSvc`) enforces Microsoft Family Safety rules such as screen time and content filters.

### Why it matters
- Without child accounts managed by Microsoft Family, the service has nothing to do.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Automatic** | `2` | Starts with Windows. |
| **Manual** | `3` | Windows default. Starts for family accounts. |
| **Disabled** | `4` | Never starts. |

> **Good to know:** Do not disable it on PCs used by children with Family Safety limits — the limits stop working.
