# NEM syntax

This page describes the concrete syntax the compiler currently accepts.

## Lexical structure

NEM source is plain UTF-8 text with standard whitespace and line breaks. The lexer recognizes:

- identifiers: names like `main`, `greet`, `value`
- integer literals: `0`, `42`, `123`
- string literals: `"hello"`
- boolean literals: `true`, `false`
- punctuation: `{ } ( ) , ; : -> =`
- operators: `+ - * / % == != < <= > >= && || !`
- keywords: `fn`, `let`, `mut`, `return`, `print`, `if`, `else`, `while`, `int`, `string`, `bool`, `true`, `false`

## Function declarations

A function is written as:

```text
fn name(param1: type, param2: type) -> return_type {
    ...
}
```

Example:

```text
fn add(a: int, b: int) -> int {
    return a + b;
}
```

`-> return_type` is optional. When omitted, the function returns `void`.

## Statements

### Variable declaration

```text
let name: type = expression;
let mut name: type = expression;
```

Mutable variables are introduced with `let mut`; immutable variables are created with `let`.

### Assignment

```text
name = expression;
```

Assignment is valid only for mutable variables already declared in scope.

### Conditional statement

```text
if condition {
    ...
} else {
    ...
}
```

The condition must have type `bool`.

### While loop

```text
while condition {
    ...
}
```

The loop condition must have type `bool`.

### Return statement

```text
return expression;
```

Examples:

```text
return 42;
return "hello";
return true;
```

### Print statement

The compiler recognizes `print(...)` as a statement-like call.

```text
print("Hello, world!");
```

## Expressions

The expression grammar includes:

- integer literal
- string literal
- boolean literal
- identifier
- function call
- arithmetic expressions
- comparison expressions
- boolean expressions
- parenthesized expression

Examples:

```text
add(x, y)
message
(42)
value == 0
flag && !done
```

## Semantics to keep in mind

- functions are defined at the top level
- variable names are lexically scoped
- immutable variables cannot be reassigned
- assignment requires a mutable variable in scope
- `if` and `while` conditions must evaluate to `bool`
- functions returning a non-void type must return on all control-flow paths
- `main` is the conventional entry point, and the compiler emits a C `main`-style function for it

## Example program

```text
fn is_even(value: int) -> bool {
    if value % 2 == 0 {
        return true;
    }
    return false;
}

fn main() {
    let mut total: int = 0;
    while total < 3 {
        if is_even(total) {
            print("even");
        } else {
            print("odd");
        }
        total = total + 1;
    }
}
```
