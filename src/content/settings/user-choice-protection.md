**User Choice Protection** (`UCPD.sys`) is a Windows driver that blocks other programs from changing default apps and a few protected settings.

### Why it matters
- It blocks changes to the Widgets settings and the default browser, even from administrators.
- Turning it off lets this app apply the rules that Windows otherwise rejects.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **On** | `1` | Windows default. The driver loads at startup and protects the values. |
| **Disabled** | `4` | The driver no longer loads. |

> **Good to know:** The change takes effect after a restart. Without the driver, other software can change your default apps too. The **User Choice Protection restore** scheduled task switches the driver back on unless it is disabled as well.
