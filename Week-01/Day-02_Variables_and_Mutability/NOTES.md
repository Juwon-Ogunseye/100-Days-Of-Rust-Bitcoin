# Rust Notes – Variables, Mutability, Shadowing, and Integer Types

## 1. Variables are Immutable by Default

By default, a variable's value **cannot be changed** after it is created.

```rust
let x = 5;
```

This works:

```rust
println!("{x}");
```

This does **not**:

```rust
x = 6;
```

Compiler error:

```text
cannot assign twice to immutable variable `x`
```

---

## 2. Mutable Variables (`mut`)

Use `mut` when you want to change a variable's value.

```rust
let mut x = 5;

x = 6;

println!("{x}");
```

Output:

```text
6
```

### Rule

`mut` allows you to change the **value** of a variable, **not its type**.

✅ Works:

```rust
let mut x = 5;
x = 10;
```

Both values are of type `i32`.

❌ Does not work:

```rust
let mut x = 5;
x = "hello";
```

The types are different (`i32` vs `&str`).

---

# 3. Shadowing

Shadowing means creating a **new variable** with the **same name**.

```rust
let x = 5;

let x = x + 1;

let x = x * 2;

println!("{x}");
```

Output:

```text
12
```

Each `let` creates a **new variable**.

The previous variable is hidden (shadowed).

---

## 4. Shadowing vs. Mutability

### Mutability

```rust
let mut x = 5;

x = 6;
```

- Same variable
- Same type
- Different value

---

### Shadowing

```rust
let x = 5;

let x = x + 1;
```

- New variable
- Same name
- Can have a different type

---

## 5. Why This Doesn't Work

```rust
let mut spaces = "   ";

spaces = spaces.len();
```

Rust gives the error:

```text
expected `&str`
found `usize`
```

### Why?

Initially:

```rust
let mut spaces = "   ";
```

Type:

```text
&str
```

Then:

```rust
spaces.len()
```

returns:

```text
usize
```

Rust sees:

```rust
spaces = 3;
```

This attempts to assign a `usize` to a variable whose type is `&str`, which is not allowed.

---

## 6. The Correct Way (Shadowing)

```rust
let spaces = "   ";

let spaces = spaces.len();
```

### What Happens?

First variable:

```text
spaces
Type: &str
Value: "   "
```

Second variable:

```text
spaces
Type: usize
Value: 3
```

The second variable **shadows** the first one.

---

## 7. When Should I Use `mut`?

Use `mut` when the variable's **type stays the same**.

Example:

```rust
let mut score = 0;

score = 10;

score = 20;
```

The type always remains `i32`.

---

## 8. When Should I Use Shadowing?

Use shadowing when:

- You want to reuse a variable name.
- The variable changes to a **different type**.
- You want to transform data step by step.

Example:

```rust
let input = "42";

let input: i32 = input.parse().unwrap();
```

The first `input` is a `&str`.

The second `input` is an `i32`.

---

# 9. Signed vs. Unsigned Integers

## Signed Integers (`i`)

Can store:

- Positive numbers
- Zero
- Negative numbers

Example:

```rust
let temperature: i32 = -10;
```

Think of it as:

```text
i = signed
```

Common signed integer types:

- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`

---

## Unsigned Integers (`u`)

Can store:

- Zero
- Positive numbers

Cannot store negative numbers.

Example:

```rust
let age: u32 = 25;
```

This is illegal:

```rust
let age: u32 = -25;
```

Common unsigned integer types:

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `usize`

---

## 10. Why Does `len()` Return `usize`?

```rust
let name = "Rust";

name.len()
```

returns:

```text
usize
```

because a length can never be negative.

Examples of values represented by `usize`:

- String length
- Vector length
- Array indexing
- Memory sizes

---

# 11. Memory Tricks

## `mut`

> **Same variable → Same type → Different value**

Example:

```rust
let mut x = 5;

x = 10;
```

---

## Shadowing

> **New variable → Same name → Different type allowed**

Example:

```rust
let x = "Hello";

let x = x.len();
```

---

# 12. Signed vs. Unsigned Integer Ranges

| Type | Bits | Can Be Negative? | Range |
|------|------|------------------|-------------------------------|
| `i8` | 8 | ✅ Yes | -128 to 127 |
| `u8` | 8 | ❌ No | 0 to 255 |
| `i16` | 16 | ✅ Yes | -32,768 to 32,767 |
| `u16` | 16 | ❌ No | 0 to 65,535 |
| `i32` | 32 | ✅ Yes | -2,147,483,648 to 2,147,483,647 |
| `u32` | 32 | ❌ No | 0 to 4,294,967,295 |
| `i64` | 64 | ✅ Yes | Very large positive and negative values |
| `u64` | 64 | ❌ No | Very large positive values |
| `isize` | Depends on the platform | ✅ Yes | Platform dependent |
| `usize` | Depends on the platform | ❌ No | Platform dependent |

---

# Key Takeaways

- Variables are **immutable** by default.
- Add `mut` to make a variable's **value** mutable.
- `mut` **cannot** change a variable's type.
- Shadowing (`let` with the same name) creates a **new variable**, allowing the type to change.
- `i` types are **signed** and can store negative numbers.
- `u` types are **unsigned** and can only store zero or positive numbers.
- `usize` is used for lengths, sizes, and indexes because these values cannot be negative.

---

# One-Sentence Summary

> **`mut` changes a variable's value, while shadowing (`let` with the same name) creates a new variable, allowing you to reuse the name and even change its type.**