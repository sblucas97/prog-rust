# Repository for exercises on Programming Rust 2nd

## Chapter 2

### Functions
- The "!" char marks a macro invocation not a function call
- Rust infer types only within function bodies. For function arguments and return types, it should be defined explicitly.
- The return statement is not needed if the last sentence of a function is a expression WITHOUT a semicolomn
```rust
fn foo() -> u64 {
    45_u64
}
```
### Unit tests
- `#[test]` atop functions marks them as test functions
- test functions are skipped in normal compilations
- test functions are called by running `cargo test`
- `#[test]` marker is an attribute (like annotations in Java)

```rust

#[test]
fn test_foo() {
    assert_eq!(64_u64, foo());
}
```
### Command-Line Arguments
- Traits introduced: a collection of methods that types can implement
```rust
use std::str::FromStr;
```
- `Result` value introduced. Two possible variants: Ok(v) Err(e)

### Web

### Concurrency

### Filesystems
