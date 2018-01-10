# Throws

`throws!` is a macro for easy creation per-function error enums:

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
For more, see [examples](examples).