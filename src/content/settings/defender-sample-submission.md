Decides whether **Microsoft Defender** uploads suspicious files to Microsoft for analysis.

### Why it matters
- Uploaded samples can be your own documents or scripts.
- Defender keeps scanning and blocking threats with every option.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **Always ask** | `0` | Defender asks before uploading a file. |
| **Safe samples** | `1` | Windows default. Files without personal data, such as programs, are sent automatically. |
| **Never send** | `2` | Samples are never uploaded. |
| **All samples** | `3` | Every suspicious file is sent automatically. |

> **Good to know:** Cloud protection may detect brand-new threats a little later without samples.
