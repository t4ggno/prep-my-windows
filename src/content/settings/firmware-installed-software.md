Decides whether Windows runs programs that the PC manufacturer stored in the firmware (the **Windows Platform Binary Table**, WPBT).

### Why it matters
- Some manufacturers use WPBT to install their utilities or updaters again after every clean Windows install.
- Blocking it keeps a fresh installation free of software you did not choose.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Off** | `1` | Windows ignores programs stored in the firmware. |
| **On** | `0` | Windows default. Firmware programs run at startup. |

> **Good to know:** Anti-theft software from some vendors relies on WPBT. Leave it on if you use such a service.
