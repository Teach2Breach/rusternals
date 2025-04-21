# NtSetTimer

A Rust library and example program demonstrating the use of Windows' NtSetTimer API.

## Overview

This project provides a simple interface to Windows' low-level timer functionality through the NtSetTimer API. It allows you to create timers that can trigger after a specified duration and optionally execute callback functions.

## Features

- Create one-shot timers with configurable delays
- Optional callback functions when timers fire
- Error handling for timer creation and management
- Simple library interface for integration into other projects

## Usage

### As a Library

```rust
use NtSetTimer::set_timer;
use std::time::Duration;
use std::ptr::null_mut;

// Set a timer for 2 seconds
let timer_handle = set_timer(
    Duration::from_secs(2),
    None, // Optional callback
    null_mut() // Optional context
).expect("Failed to set timer");
```

### Running the Example

The example program creates a timer that fires after 2 seconds:

```bash
cargo run --bin NtSetTimer
```

Expected output:
```
Starting NtSetTimer test...
Timer created and set successfully!
Waiting for timer to fire...
Timer fired! Result: WAIT_EVENT(0)
```

## Requirements

- Rust 2024 edition
- Windows OS
- Windows SDK

## License

This project is licensed under the MIT License. 