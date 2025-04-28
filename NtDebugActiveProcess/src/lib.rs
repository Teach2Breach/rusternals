#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use winapi::{
    shared::{
        minwindef::BOOL,
        ntdef::{HANDLE, NTSTATUS},
    },
    um::winnt::ACCESS_MASK,
};

use std::ffi::c_void;

#[macro_use]
extern crate litcrypt;

use_litcrypt!();

type GetCurrentProcessFn = unsafe extern "system" fn() -> HANDLE;

// Add this type definition near your other ones
type NtOpenProcessTokenFn = unsafe extern "system" fn(
    ProcessHandle: HANDLE,
    DesiredAccess: ACCESS_MASK,
    TokenHandle: *mut HANDLE,
) -> NTSTATUS;

// Add these type definitions at the top
#[repr(C)]
struct TOKEN_PRIVILEGES {
    PrivilegeCount: u32,
    Privileges: [LUID_AND_ATTRIBUTES; 1],
}

#[repr(C)]
struct LUID {
    LowPart: u32,
    HighPart: i32,
}

#[repr(C)]
struct LUID_AND_ATTRIBUTES {
    Luid: LUID,
    Attributes: u32,
}

const SE_PRIVILEGE_ENABLED: u32 = 0x00000002;

type NtAdjustPrivilegesTokenFn = unsafe extern "system" fn(
    TokenHandle: HANDLE,
    DisableAllPrivileges: BOOL,
    NewState: *const TOKEN_PRIVILEGES,
    BufferLength: u32,
    PreviousState: *mut TOKEN_PRIVILEGES,
    ReturnLength: *mut u32,
) -> NTSTATUS;

const TOKEN_ADJUST_PRIVILEGES: u32 = 0x0020;
const TOKEN_QUERY: u32 = 0x0008;

// Add near your other type definitions
type NtCloseFn = unsafe extern "system" fn(Handle: HANDLE) -> NTSTATUS;

// Add with other type definitions at the top
type LookupPrivilegeValueFn = unsafe extern "system" fn(
    lpSystemName: *const i8,
    lpName: *const i8,
    lpLuid: *mut LUID,
) -> BOOL;

// Add with other constants
const SE_DEBUG_NAME: &str = "SeDebugPrivilege\0";

pub fn elevate_debug(
    ntdll: *const c_void,
    kernel32: *const c_void,
    advapi32: *const c_void,
) -> Result<(), Box<dyn std::error::Error>> {
    //locate NtOpenProcessToken
    let nt_open_process_token = noldr::get_function_address(ntdll, &lc!("NtOpenProcessToken"))
        .unwrap_or_else(|| std::ptr::null_mut());

    if nt_open_process_token.is_null() {
        println!("[!] Failed to get NtOpenProcessToken address");
        return Err("NtOpenProcessToken address is null".into());
    }
    println!(
        "[+] NtOpenProcessToken address: {:?}",
        nt_open_process_token
    );

    let nt_open_process_token: NtOpenProcessTokenFn =
        unsafe { std::mem::transmute(nt_open_process_token) };

    let mut token_handle: HANDLE = std::ptr::null_mut();

    let get_current_process = noldr::get_function_address(kernel32, &lc!("GetCurrentProcess"))
        .unwrap_or_else(|| std::ptr::null_mut());
    println!("[+] GetCurrentProcess address: {:?}", get_current_process);

    let current_process: GetCurrentProcessFn = unsafe { std::mem::transmute(get_current_process) };

    println!("[*] Calling NtOpenProcessToken");
    let status = unsafe {
        //println!("[*] About to make the call");
        let result = nt_open_process_token(
            current_process(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token_handle,
        );
        //println!("[*] Call completed");
        result
    };

    if status != 0 {
        println!("[!] NtOpenProcessToken failed with status: {}", status);
        return Err("Failed to open process token".into());
    }

    if token_handle.is_null() {
        println!("[!] Token handle is null despite successful call");
        return Err("Received null token handle".into());
    }

    println!("[+] Successfully opened process token: {:?}", token_handle);

    let lookup_privilege_value =
        noldr::get_function_address(advapi32, &lc!("LookupPrivilegeValueA"))
            .unwrap_or_else(|| std::ptr::null_mut());
    println!(
        "[+] LookupPrivilegeValueA address: {:?}",
        lookup_privilege_value
    );

    let mut luid = LUID {
        LowPart: 0,
        HighPart: 0,
    };

    let success = unsafe {
        let lookup_privilege_value: LookupPrivilegeValueFn =
            std::mem::transmute(lookup_privilege_value);
        lookup_privilege_value(
            std::ptr::null(),
            SE_DEBUG_NAME.as_ptr() as *const i8,
            &mut luid,
        )
    };

    if success == 0 {
        println!("[!] LookupPrivilegeValue failed");
        std::process::exit(1);
    }

    let priv_struct = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Luid: luid,
            Attributes: SE_PRIVILEGE_ENABLED,
        }],
    };

    let nt_adjust_privileges_token = noldr::get_function_address(ntdll, &lc!("NtAdjustPrivilegesToken"))
        .unwrap_or_else(|| std::ptr::null_mut());
    println!(
        "[+] NtAdjustPrivilegesToken address: {:?}",
        nt_adjust_privileges_token
    );

    let status = unsafe {
        let nt_adjust_privileges_token: NtAdjustPrivilegesTokenFn =
            std::mem::transmute(nt_adjust_privileges_token);
        println!("[*] Attempting to adjust token privileges...");
        nt_adjust_privileges_token(
            token_handle,
            0,
            &priv_struct,
            std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    if status != 0 {
        if status == 0x106 {
            println!("[!] This program must be run as Administrator");
            std::process::exit(1);
        }
        println!(
            "[!] NtAdjustPrivilegesToken failed with status code: {:#x}",
            status
        );
        std::process::exit(1);
    }

    println!("[+] Successfully enabled SeDebugPrivilege!");

    // After you're done with the token_handle:
    let nt_close =
        noldr::get_function_address(ntdll, &lc!("NtClose")).unwrap_or_else(|| std::ptr::null_mut());

    let status = unsafe {
        let nt_close: NtCloseFn = std::mem::transmute(nt_close);
        nt_close(token_handle)
    };

    if status != 0 {
        println!("[!] NtClose failed with status: {}", status);
        return Err("Failed to close token handle".into());
    }

    println!("[+] Successfully closed token handle");

    Ok(())
}