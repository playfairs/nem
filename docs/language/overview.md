# NEM language overview

NEM is a small typed native language designed to compile through a conventional frontend pipeline into native code.

## Core ideas

- NEM is its own language, not C with different syntax.
- Functions may declare parameter types and return types.
- Variables are lexically scoped.
- Name resolution and semantic validation happen before lowering.
- The compiler lowers typed constructs into NEM IR and then emits backend code.

## Example

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
