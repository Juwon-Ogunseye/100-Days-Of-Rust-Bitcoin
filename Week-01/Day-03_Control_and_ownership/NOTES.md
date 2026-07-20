# Day 3 — Control Flow
**Reading:** rust-book.cs.brown.edu — Control Flow (if expressions, loops)

## What I understood

### if expressions
- `if` branches code based on a condition: `if cond { } else { }`
- The condition **must** be a `bool` — Rust won't auto-convert integers or 
  strings into booleans like some other languages do
- `else if` lets you chain multiple conditions. Rust checks each one in 
  order and stops at the first `true` match — it doesn't check the rest
- Since `if` is an expression (not just a statement), its result can be 
  assigned directly:
```rust
  let number = if condition { 5 } else { 6 };
```
  Both arms must return the **same type**, or it won't compile

### Loops — three kinds
- **`loop`** — runs forever until `break` is called. Can return a value:
```rust
  let result = loop {
      counter += 1;
      if counter == 10 {
          break counter * 2;
      }
  };
```
- **`while`** — runs while a condition stays true. Cleaner than manually 
  combining `loop` + `if` + `break`
- **`for`** — iterates over a collection directly:
```rust
  for element in a {
      println!("the value is: {element}");
  }
```
  Safer and faster than indexing with `while` — no risk of going out of 
  bounds, and no per-iteration bounds check

### Loop labels
- In nested loops, `break`/`continue` apply to the innermost loop by default
- Label an outer loop with `'name:` to target it specifically:
```rust
  'counting_up: loop {
      loop {
          if count == 2 {
              break 'counting_up;
          }
      }
  }
```

### Countdown idiom
- `for number in (1..4).rev()` is the idiomatic way to count down — 
  preferred over a manual `while` loop

## What I built
Temperature converter (Fahrenheit → Celsius) using `if`/read input pattern 
from stdin, `.parse()`, and `println!` formatting.