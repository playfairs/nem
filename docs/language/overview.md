# NEM language overview

NEM is a compact typed language designed to compile through a conventional frontend pipeline: tokenize → parse → validate semantics → lower to IR → emit native C.

## The current language shape

The implementation in the `nemc` repository defines the actual source of truth for the language. The current language surface is intentionally modest but coherent:

- function declarations with a name, parameters, and an optional return type
- scalar types: `int`, `string`, and `bool`
- variable declarations with `let` and `let mut`
- literal values for integers, strings, and booleans
- arithmetic and comparison expressions
- boolean operators: `&&`, `||`, and `!`
- assignment to mutable variables via `=`
- `if` / `else` blocks and `while` loops
- `return` statements with control-flow validation
- lexical scoping and semantic checks for type and mutability correctness

This is not yet a complete general-purpose language, but it is a coherent and testable language core.

## Example

```text
fn is_even(value: int) -> bool {
    if value % 2 == 0 {
        return true;
    }
    return false;
}

fn main() {
    let mut i: int = 0;
    while i < 5 {
        if is_even(i) {
            print("even");
        } else {
            print("odd");
        }
        i = i + 1;
    }
}
```

This example matches the supported NEM surface: booleans, conditionals, loops, and mutable state are all part of the language contract.

## Design principles

1. Keep the language small enough to reason about clearly.
2. Make the compiler frontend explicit and predictable.
3. Treat the language docs as the canonical contract for users.
4. Keep the implementation and specification separate, while ensuring they stay aligned.

## Project boundaries

- `nem` defines the language, examples, docs, and user-facing runtime.
- `nemc` contains the actual compiler details and regression tests.
- `nox` provides the build system used to compile the compiler toolchain.

The docs in this repository should be read as the language specification for the user-facing project, while the compiler project remains the implementation authority.

