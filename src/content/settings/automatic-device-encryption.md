Controls whether Windows turns on **device encryption** (BitLocker) automatically, for example during setup or when you sign in with a Microsoft account.

### Why it matters
- Automatic encryption ties the recovery key to your Microsoft account; losing access to it can lock you out of your data.
- Drive encryption costs a little performance on some SSDs.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Off** | `1` | Windows no longer encrypts drives on its own. |
| **On** | `0` | Windows default. Windows encrypts supported devices automatically. |

> **Good to know:** This rule is optional and does not decrypt drives that are already encrypted. Encryption protects your data if the device is lost or stolen — keep it on for laptops.
