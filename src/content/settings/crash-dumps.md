Limits **crash dump collection** for diagnostics.

### Why it matters
- Memory dumps can contain fragments of open documents, passwords or browser content.
- With the limit, Windows only collects small dumps instead of full memory images.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Off** | `1` | Dump collection for diagnostics is limited. |
| **On** | `0` | Windows default. Larger dumps may be collected with optional data. |

> **Good to know:** Local crash dumps for your own troubleshooting are still written as configured in *System properties → Startup and Recovery*.
