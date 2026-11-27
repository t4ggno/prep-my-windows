**SysMain** (formerly *Superfetch*) learns which apps you use and preloads them into memory.

### Why it matters
- On fast SSDs the benefit is small, while the service causes background disk and CPU activity.
- On hard drives it can make app starts noticeably faster.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Automatic** | `2` | Windows default. Starts with Windows. |
| **Manual** | `3` | Starts only when requested. |
| **Disabled** | `4` | Never starts and stops preloading. |

> **Good to know:** This rule is optional. Turn it on if you see constant disk activity from SysMain, leave it off on PCs with a hard drive.
