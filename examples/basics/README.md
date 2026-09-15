# NEM basics examples

This directory contains the canonical introductory examples for the NEM language.

## Hello world

```text
fn main() {
    print("Hello, world!");
}
```

This is the minimal valid NEM program accepted by the compiler and launcher.

## Control flow and booleans

```text
fn is_even(value: int) -> bool {
    if value % 2 == 0 {
        return true;
    }
    return false;
}

fn main() {
    let mut x: int = 0;
    while x < 4 {
        if is_even(x) {
            print("even");
        } else {
            print("odd");
        }
        x = x + 1;
    }
}
```

This example demonstrates the language milestone: boolean expressions, comparison operators, `if`/`else`, loops, and mutable state.
