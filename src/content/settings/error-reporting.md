**Windows Error Reporting** sends crash and hang reports to Microsoft and may look up solutions.

### Why it matters
- Reports can include memory contents and file paths of the crashed app.
- Report uploads cause background activity after every crash.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Off** | `1` | No error reports are created or sent. |
| **On** | `0` | Windows default. Reports are sent to Microsoft. |

> **Good to know:** Event Viewer still logs every crash locally.
