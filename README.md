# About
Format integers on stack
in binary, octal, decimal or hexadecimal.

# Examples
```rust
use fmt_int::{FmtInto, fmt_int};

fn main() {
    let num: u8 = 123;
    assert_eq!(fmt_int!(num), format!("{}", num));
    assert_eq!(fmt_int!("{:b}", num), format!("{:b}", num));
    assert_eq!(fmt_int!("{:o}", num), format!("{:o}", num));
    assert_eq!(fmt_int!("{:x}", num), format!("{:x}", num));
    assert_eq!(fmt_int!("{:X}", num), format!("{:X}", num));
}
```
Or you can do it explicitly
```rust
use fmt_int::{FmtInto, NumBuffer, Binary};

fn main() {
    let mut buf = NumBuffer::<Binary, _>::new();
    _ = 123u8.fmt_into(&mut buf);
}
```

# Advantage
Currently the standard library doesn't decouple
the formatting of integers (in binary, octal or hexadecimal)
with [`Formatter`](https://doc.rust-lang.org/1.98.0/core/fmt/struct.Formatter.html),
which sometimes causes performance penalty.

With this crate, you can do
```rust
use fmt_int::*;
["bytes: ", fmt_int!("{:b}", 123u8)].concat();
```
which is faster and more memory efficient than
```rust
format!("bytes: {:b}", 123);
```

# Similar crates
- [`numtoa`](https://docs.rs/numtoa)
- Fast floats formatting: [`zmij`](https://docs.rs/zmij)
