### Defferences between variables and constants
1. You aren't allowed to use ```mut``` with constants. Constants aren't just immutable by default - they're always immutable
2. Defference is that constants may be set only to a constant expression, not to the result of a function call or any other value that could only be computed at runtime.

### Shadowing
Rustanceans say that the first variable is *shadowed* by the second, which means that the second variable's value is what appears when the variable is used.

