# `is_hawk_tuah`

This Rust crate provides a single function, `is_hawk_tuah`, which checks if a
string contains a hawk, followed by a `too` or a `two`.

Inspired by the Hailey Welch, and the legendary [is-even](https://www.npmjs.com/package/is-even) library.

## Usage

```rust
use is_hawk_tuah::is_hawk_tuah;

assert!(is_hawk_tuah("hawk too").unwrap());
assert!(is_hawk_tuah("hawk two").unwrap());
assert!(is_hawk_tuah("hawk to").unwrap());
assert!(is_hawk_tuah("hawk tuah").unwrap());
```
