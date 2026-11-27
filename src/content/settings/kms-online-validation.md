Turns off **online validation** for KMS clients (Key Management Service activation).

### Why it matters
- Prevents the KMS client from sending licence data to Microsoft's online activation service.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Off** | `1` | No online validation for KMS activation. |
| **On** | `0` | Windows default. Online validation is used. |

> **Good to know:** Has no effect on PCs activated with a retail or OEM licence.
