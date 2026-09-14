# NEM

NEM is the native programming language itself.

This repository defines the language, its examples, and its language-level documentation. It does not contain the compiler implementation; that lives in the separate `nemc` repository.

## Purpose

The language project answers the question:

> What is the NEM language?

The compiler project answers:

> How does the official NEM compiler implement that language?

## Current language shape

NEM is a small typed language with a frontend pipeline based on tokens, parsing, semantic analysis, and IR lowering. The language includes concepts such as:

- typed function declarations
- typed parameters
- return types
- integer and string values
- variables and lexical scopes
- function calls
- basic semantic validation

A simple example is:

```text
fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() {
    let x: int = 20;
    let y: int = 22;
    let result: int = add(x, y);
    print(result);
}
```

## Repository separation

- `nem`: language definition, documentation, examples, and language-level materials
- `nemc`: official compiler implementation, lexer, parser, AST, NEMantics, diagnostics, IR, backend, and tests
- `nox`: build system used to build the compiler

## Development notes

The language repository remains a specification and examples project rather than a compiler source tree. The implementation-specific code and generated build artifacts live in the compiler repository.
