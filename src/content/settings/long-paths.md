Allows apps to use file paths longer than the classic limit of **260 characters**.

### Why it matters
- Deep project folders, archives and developer tools (Node.js, Git) often exceed 260 characters.
- Apps must support long paths themselves; the setting removes the Windows-wide limit.

### Values
| Option | Value | Meaning |
| --- | --- | --- |
| **On** | `1` | Apps that support long paths can use them. |
| **Off** | `0` | Windows default. Paths stay limited to 260 characters. |
