# NEM semantics

The semantic model of NEM is intentionally simple and matches the behavior of the official compiler implementation.

## Type system

NEM currently supports the following scalar types:

- `int`
- `string`
- `bool`
- `void`

`void` is used for functions that do not return a value. The compiler validates return and initializer types against the declared signature.

## Boolean semantics

Booleans are first-class values in NEM. The language supports:

- boolean literals: `true` and `false`
- comparison expressions returning `bool`
- boolean connectives: `&&`, `||`, and `!`
- `if` and `while` conditions that must evaluate to `bool`

A comparison such as `x == y` or `x < y` produces a boolean result. Boolean operators require boolean operands.

## Mutability and assignment

A variable declaration without `mut` creates an immutable binding. A declaration with `let mut` creates a mutable binding. The compiler rejects assignment to immutable variables.

Assignment checks both name resolution and type compatibility. The assigned value must match the declared type of the variable.

## Scoping

Variables are lexically scoped. A function body introduces a local scope, and symbols from outer scopes are visible unless shadowed by a local declaration.

## Function model

Functions are defined as named declarations with parameters and return types. Each function is entered into a global symbol table for name lookup during validation.

## Control-flow validation

The semantic pass checks:

- function declarations are valid
- function names are present
- parameter names and local names are tracked in scope
- initializer types match declared variable types when a type is explicitly stated
- assignment targets are mutable and defined
- condition types are booleans in `if` and `while`
- return expressions match the declared return type
- non-void functions return on all reachable control-flow paths
- calls to `print` have a valid argument shape

## Missing return analysis

For functions whose declared return type is not `void`, the compiler requires a return path on every control-flow branch. For example, this is rejected:

```text
fn foo(x: int) -> int {
    if x > 0 {
        return 1;
    }
}
```

The function has an `if` with no `else`, so one control-flow path exits without returning. The semantic pass reports a missing-return diagnostic.

## Example validation

```text
fn is_even(value: int) -> bool {
    if value % 2 == 0 {
        return true;
    }
    return false;
}
```

This is valid: the function returns a `bool` on both branches and the condition is boolean.

## Editorial note

This repository documents the language as it exists today, not a hypothetical future version. The purpose is to keep the language project honest: the examples and semantics should reflect the actual compiler behavior.
