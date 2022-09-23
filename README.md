# Debug lib

Print debug messages if the DEBUG environment variable is set

# Usage

```rust
use debug::debug;

fn main() {
    println!("this will be printed every time");
    debug("this will only be printed if the DEBUG env var is set");
}
```

For more code examples check out the usage.rs example.

# License

This project is distributed under the MIT license.

