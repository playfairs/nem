# NEM

NEM is the language project for the NEM native programming language.

This repository is the canonical home for:

- the language definition
- example programs
- user-facing documentation
- the runtime launcher that invokes the compiler

The implementation of the compiler itself lives in the separate `nemc` repository, while `nox` provides the build system used by the compiler tooling.

## What NEM is today

The official compiler now supports the following language surface:

- function declarations with typed parameters and optional return types
- `int`, `string`, and `bool` scalar types
- mutable and immutable local variables using `let mut` and `let`
- integer literals, string literals, and boolean literals (`true`, `false`)
- comparison operators: `==`, `!=`, `<`, `<=`, `>`, `>=`
- boolean operators: `&&`, `||`, `!`
- assignment with `=` for mutable variables
- `if ... else` conditionals and `while` loops
- `return` statements and flow-sensitive missing-return checking
- lexical scope and semantic validation for types and mutability

This repository documents the language as it exists, not a larger design wishlist.

## Example

```text
fn is_even(value: int) -> bool {
    if value % 2 == 0 {
        return true;
    }
    return false;
}

fn main() {
    let mut x: int = 0;
    while x < 5 {
        if is_even(x) {
            print("even");
        } else {
            print("odd");
        }
        x = x + 1;
    }
}
```

The example above reflects the current language shape and the behavior validated by the compiler project.

## Repository roles

- `nem`: language docs, examples, project pages, and runtime launcher
- `nemc`: compiler source code, lexer, parser, semantics, IR lowering, backend, and tests
- `nox`: build system used across the toolchain

## Quickstart

Install the compiler and then run a NEM program with the launcher:

```bash
nem hello.nem
```

The launcher resolves `nemc` from `NEMC` or from the environment `PATH`. If the compiler is not installed, it exits with a clear message telling you how to configure it.

## Docs

- [docs/README.md](docs/README.md)
- [docs/language/overview.md](docs/language/overview.md)
- [docs/language/syntax.md](docs/language/syntax.md)
- [docs/language/semantics.md](docs/language/semantics.md)

## Examples

The canonical examples are kept under [examples/basics](examples/basics).
