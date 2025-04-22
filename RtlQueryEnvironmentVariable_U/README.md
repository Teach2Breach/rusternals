# RtlQueryEnvironmentVariable_U

A Rust program that demonstrates how to query environment variables using the Windows Native API function `RtlQueryEnvironmentVariable_U`.

## Features

- Query environment variables using Windows Native API
- Built-in helper function for retrieving username from environment variables.
- Safe handling of Unicode strings and Windows-specific types

## Example Usage

```rust
use RtlQueryEnvironmentVariable_U::{get_environment_variable, get_uuid};

fn main() {
    // Query a specific environment variable
    match get_environment_variable("OneDrive") {
        Ok(Some(value)) => println!("OneDrive: {}", value),
        Ok(None) => println!("OneDrive not found"),
        Err(e) => eprintln!("Error getting OneDrive: {}", e),
    }

        match get_environment_variable("USERNAME") {
        Ok(Some(value)) => println!("USERNAME: {}", value),
        Ok(None) => println!("USERNAME not found"),
        Err(e) => eprintln!("Error getting USERNAME: {}", e),
    }

}
```

## Sample Output

```
OneDrive: C:\Users\adversary\OneDrive
USERNAME: adversary
```

## Dependencies

- `windows` - For Windows API bindings
- `ntapi` - For Native API definitions

## Building

```bash
cargo build --bin RtlQueryEnvironmentVariable_U
```

## Running

```bash
cargo run --bin RtlQueryEnvironmentVariable_U
``` 