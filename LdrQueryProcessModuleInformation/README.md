# LdrQueryProcessModuleInformation

A Rust program that demonstrates the use of the Windows `LdrQueryProcessModuleInformation` function to enumerate loaded modules in a process.

## Description

This program uses the undocumented Windows API function `LdrQueryProcessModuleInformation` to list all modules (DLLs and executables) loaded in the current process. For each module, it displays:
- Module name and path
- Base address in memory
- Size in bytes

## Usage

```bash
cargo run --bin LdrQueryProcessModuleInformation
```

## Example Output

```
Loaded modules:

Module 1:
  Name: C:\path\to\program.exe
  Path: C:\path\to\program.exe
  Base Address: 0x7FF7F64A0000
  Size: 221184 bytes

Module 2:
  Name: C:\WINDOWS\SYSTEM32\ntdll.dll
  Path: C:\WINDOWS\SYSTEM32\ntdll.dll
  Base Address: 0x7FFE42EE0000
  Size: 2490368 bytes

[... additional modules ...]
```

## Requirements

- Windows operating system
- Rust toolchain

## Building

```bash
cargo build --bin LdrQueryProcessModuleInformation
```

For a release build with optimizations:

```bash
cargo build --bin LdrQueryProcessModuleInformation --release
``` 