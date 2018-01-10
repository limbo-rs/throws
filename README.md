# Throws

`throws!` is a macro for easy creation per-function error enums.

## Example

```rust
throws!(FileDoubleError = Io(io::Error), Parse(ParseIntError));
fn file_double(path: &str) -> Result<i32, FileDoubleError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let n: i32 = contents.trim().parse()?;

    Ok(2 * n)
}
```
For more, check out the [examples](/examples) directory!

You can run an individual example using `cargo run --example example-name`.

## Installation

If you're using cargo, just add `throws` to your `Cargo.toml`:

```toml
[dependencies]

throws = "*"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## License

MIT
