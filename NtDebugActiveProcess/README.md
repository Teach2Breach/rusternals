# NtDebugActiveProcess

A Rust library and example program demonstrating the use of Windows' NtDebugActiveProcess API.

## Overview

This project provides a simple interface to Windows' low-level debugging functionality through the NtDebugActiveProcess API. It demonstates how to attach a debugger to a running process.

## Features

* Elevates debug privileges using NtOpenProcessToken and NtAdjustPrivilegesToken
* Creates a debug object using NtCreateDebugObject
* Attaches to a process using NtDebugActiveProcess
* Proper cleanup of debug objects and handles

## Usage

### Running the Example

The example program creates a notepad process and attaches a debugger to it:

```powershell
cargo run --bin NtDebugActiveProcess
```

Expected output:
```
Attempting to elevate debug privileges...
[+] TEB address: 0xa07bca6000
[+] ntdll.dll address: 0x7ff89c8e0000
[+] kernel32.dll address: 0x7ff89be20000
[+] advapi32.dll handle: HMODULE(140705745141760)
[+] advapi32.dll address: 0x7ff89bf50000
[+] NtOpenProcessToken address: 0x7ff89ca3e600
[+] GetCurrentProcess address: 0x7ff89be44970
[*] Calling NtOpenProcessToken
[+] Successfully opened process token: 0xd8
[+] LookupPrivilegeValueA address: 0x7ff89bf6e700
[+] NtAdjustPrivilegesToken address: 0x7ff89ca3c7d0
[*] Attempting to adjust token privileges...
[+] Successfully enabled SeDebugPrivilege!
[+] Successfully closed token handle
[+] Creating notepad process...
[+] Process handle: 0x2d4
[+] Debug object handle: 0xc000000d
[+] Debug active process: 0x7ff89ca3da60
[+] Process ID: 29512
[+] Press Enter to clean up and exit...

[+] Process exited
```

## Requirements

* Rust 2024 edition
* Windows OS
* Administrator privileges (required for debug privileges)

## License

This project is licensed under the MIT License. 