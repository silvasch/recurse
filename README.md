# recurse

Recursively iterate through a directory.

## Installation

```bash
cargo add --git https://github.com/silvasch/recurse
```

## Usage

```rust
for file in recurse::Recurse::new(".") {
    let file = file.unwrap();
    println!("{}", file.display());
}
```

Take a look at the [examples directory](https://github.com/silvasch/recurse/tree/main/examples)
for a more complex example.
