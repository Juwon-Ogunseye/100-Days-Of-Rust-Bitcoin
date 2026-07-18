# Day 1 — Guessing Game
**Reading:** rust-book.cs.brown.edu ch02 - Guessing Game Tutorial

## What I understood
- `let mut` vs `let` — variables are immutable by default
- `Result<T, E>` from `.parse()` — need `.expect()` or `match` to handle it
- `rand::thread_rng().gen_range()` for random number generation
- `cmp::Ordering` enum for comparing guesses

## Concepts learned

- **Library crate**: contains code meant to be used within other code — it can't 
  be executed on its own (unlike a binary crate).
- **Semantic Versioning (SemVer)**: `rand = "0.8.5"` means "at least 0.8.5, 
  but below 0.9.0."
- **Reproducible builds**: the first `cargo build` creates `Cargo.lock`, which 
  locks the exact dependency versions used. Future builds stay locked to those 
  versions until you deliberately update the crate version.
- **Traits/methods**: crates expose methods via traits. Use `cargo doc --open` 
  to browse a crate's docs and see what's available.
- **Default integer type**: if you don't annotate a number's type, Rust infers 
  `i32` by default.
- **Shadowing**: lets you re-declare a variable with the same name (can even 
  change its type). The old value isn't destroyed — it's just shadowed within 
  that scope.

## What I built
Guessing game with input validation and a loop until correct guess.