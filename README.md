split-rs
========

```toml
[dependencies.split]
git = "https://github.com/sbward/split-rs"
```

```rust
use split::split;

fn main() {
	let list = [0u, ..1000];

	let slices = split(list.as_slice(), 10); // returns 10 slices of 100 items each.

	// ...
}
```
