The **Wallet** service (`WalletService`) hosts objects used by Microsoft wallet and payment clients.

### Why it matters
- Not needed unless an app explicitly uses Microsoft Wallet.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Automatic** | `2` | Starts with Windows. |
| **Manual** | `3` | Windows default. Starts when a wallet app needs it. |
| **Disabled** | `4` | Never starts. |
