# flatten-dir

This simple library and CLI flattens a directory recursively.

## CLI

### Installation

```sh
cargo install flatten_dir
```

### Usage

```sh
flatten_dir <path>
```

## Library

### Installation

```toml
# In Cargo.toml
[dependencies]
flatten-dir = "0.1"
```

### Usage

```rust
use flatten_dir::flatten;

fn main() {
	flatten("./music_lib_from_2001").unwrap();
}
```
