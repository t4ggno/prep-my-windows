The **Windows AI Fabric Service** (`WSAIFabricSvc`) runs local AI models for features like Recall and Click to Do.

### Why it matters
- Set to start automatically, it loads AI components at every boot even if you never use them.
- **Manual** lets it start only when an AI feature needs it.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Automatic** | `2` | Windows default. Starts with Windows. |
| **Manual** | `3` | Starts when an AI feature requests it. |
| **Disabled** | `4` | Never starts; AI features that need it stop working. |

> **Good to know:** Requires Windows 11 24H2 (build 26100) or newer.
