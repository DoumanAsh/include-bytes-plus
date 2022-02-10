# include-bytes-plus

[![Crates.io](https://img.shields.io/crates/v/include-bytes-plus.svg)](https://crates.io/crates/include-bytes-plus)
[![Documentation](https://docs.rs/include-bytes-plus/badge.svg)](https://docs.rs/crate/include-bytes-plus/)
[![Build](https://github.com/DoumanAsh/include-bytes-plus/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/include-bytes-plus/actions?query=workflow%3ARust)


Improved version of Rust's `include_bytes` macro that allows to reinterpret input as differently typed array.

Due to inability to capture current file path in the stable Rust, this macro only accepts paths relative to crate's root.

# Supported types:

- Primitive fixed sized unsigned integers;

- Arrays with unsigned integers;

# Usage:

```rust
use include_bytes_plus::include_bytes;

let bytes = include_bytes!("tests/include.in");
let bytes_u16 = include_bytes!("tests/include.in" as u16);
let bytes_u16_2 = include_bytes!("tests/include with whitespaces.in" as u16);
let bytes_u16_3 = include_bytes!("tests/include with whitespaces.in" as [u8; 48]);
let bytes_u16_4 = include_bytes!("tests/include with whitespaces.in" as [u16; 12]);

assert_eq!(bytes.len(), bytes_u16.len() * 2);
assert_eq!(bytes.len(), bytes_u16.len() * 2);
assert_eq!(bytes.len(), bytes_u16_2.len() * 2);
assert_eq!(bytes_u16_3.len(), 1);
assert_eq!(bytes_u16_3[0].len(), 48);
assert_eq!(bytes_u16_4.len(), 2);
assert_eq!(bytes_u16_4[0].len(), 12);
assert_eq!(bytes_u16_4[1].len(), 12);
```

# Debugging timings:

Set env variable `RUST_INCLUDE_BYTES_LOG=1` to enable logging of each parsed file statistics
