#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use noldr::{self, HMODULE, IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY, IMAGE_NT_HEADERS64, TEB};
use NtDebugActiveProcess::elevate_debug;
use std::ffi::c_void;

#[macro_use]
extern crate litcrypt;

type IMAGE_NT_HEADERS = IMAGE_NT_HEADERS64;

use_litcrypt!();
fn main() {
    println!("Attempting to elevate debug privileges...");
    let teb: *const TEB = noldr::get_teb();
    println!("[+] TEB address: {:?}", teb);

    let ntdll = noldr::get_dll_address(lc!("ntdll.dll").to_string(), teb).unwrap();
    println!("[+] ntdll.dll address: {:?}", ntdll);

    let kernel32 = noldr::get_dll_address(lc!("kernel32.dll").to_string(), teb).unwrap();
    println!("[+] kernel32.dll address: {:?}", kernel32);


    //load advapi.dll
    let advapi32 = load_dll("advapi32.dll", kernel32);
    println!("[+] advapi32.dll handle: {:?}", advapi32);
    //deref the handle to get the base address
    let advapi32_base = unsafe { std::mem::transmute::<HMODULE, *const c_void>(advapi32) };
    println!("[+] advapi32.dll address: {:?}", advapi32_base);

    elevate_debug(ntdll, kernel32, advapi32_base).unwrap();

    //Create a process to debug
    println!("[+] Creating notepad process...");
    let process_name = "notepad.exe";

    let process_handle = create_process(process_name);

    //if process handle is null, print an error and exit
    if process_handle.is_null() {
        println!("[-] Failed to create process");
        return;
    }
    println!("[+] Process handle: {:?}", process_handle);

    //Debug the process by creating a debug object with NtCreateDebugObject and NtDebugActiveProcess
    let debug_object = noldr::get_function_address(ntdll, &lc!("NtCreateDebugObject")).unwrap();
    let debug_object: extern "system" fn() -> winapi::shared::ntdef::HANDLE =
        unsafe { std::mem::transmute(debug_object) };
    let debug_object_handle = debug_object();
    println!("[+] Debug object handle: {:?}", debug_object_handle);

    //Debug the process
    let debug_active_process = noldr::get_function_address(ntdll, &lc!("NtDebugActiveProcess")).unwrap();
    let debug_active_process: extern "system" fn(winapi::shared::ntdef::HANDLE) -> bool =
        unsafe { std::mem::transmute(debug_active_process) };
    debug_active_process(process_handle);

    //print some helpful info
    println!("[+] Debug active process: {:?}", debug_active_process);

    //get the process id using GetProcessId
    let get_process_id = noldr::get_function_address(kernel32, &lc!("GetProcessId")).unwrap();
    let get_process_id: extern "system" fn(winapi::shared::ntdef::HANDLE) -> u32 =
        unsafe { std::mem::transmute(get_process_id) };
    let process_id = get_process_id(process_handle);
    println!("[+] Process ID: {:?}", process_id);

    //pause here and wait for user input
    println!("[+] Press Enter to clean up and exit...");
    let _ = std::io::stdin().read_line(&mut String::new());

    //call NtRemoveProcessDebug
    let nt_remove_process_debug = noldr::get_function_address(ntdll, &lc!("NtRemoveProcessDebug")).unwrap();
    let nt_remove_process_debug: extern "system" fn(winapi::shared::ntdef::HANDLE) -> bool =
        unsafe { std::mem::transmute(nt_remove_process_debug) };
    nt_remove_process_debug(process_handle);

    //call NtTerminateProcess
    let nt_terminate_process = noldr::get_function_address(ntdll, &lc!("NtTerminateProcess")).unwrap();
    let nt_terminate_process: extern "system" fn(winapi::shared::ntdef::HANDLE, winapi::shared::minwindef::DWORD) -> bool =
        unsafe { std::mem::transmute(nt_terminate_process) };
    nt_terminate_process(process_handle, 0);

    //close the debug object handle
    let close_debug_object = noldr::get_function_address(ntdll, &lc!("NtClose")).unwrap();
    let close_debug_object: extern "system" fn(winapi::shared::ntdef::HANDLE) -> bool =
        unsafe { std::mem::transmute(close_debug_object) };
    close_debug_object(debug_object_handle);

    //close the process handle
    let close_handle = noldr::get_function_address(ntdll, &lc!("NtClose")).unwrap();
    let close_handle: extern "system" fn(winapi::shared::ntdef::HANDLE) -> bool =
        unsafe { std::mem::transmute(close_handle) };
    close_handle(process_handle);

    //print some helpful info
    println!("[+] Process exited");    
    
}

//for loading the dll and getting a handle to it
pub fn load_dll(dll_name: &str, kernel32_base: *const c_void) -> HMODULE {
    unsafe {
        // Get the base address of kernel32.dll
        //let kernel32_base = get_dll_address("kernel32.dll".to_string(), get_teb()).unwrap();

        // Get the address of LoadLibraryA function
        let load_library_a = get_function_address(kernel32_base, &lc!("LoadLibraryA")).unwrap();
        let load_library_a: extern "system" fn(*const i8) -> HMODULE =
            std::mem::transmute(load_library_a);

        // Convert dll_name to a C-style string
        let c_dll_name = std::ffi::CString::new(dll_name).unwrap();

        // Call LoadLibraryA to get the handle
        load_library_a(c_dll_name.as_ptr())
    }
}

//get the address of a function in a dll
pub fn get_function_address(dll_base: *const c_void, function_name: &str) -> Option<*const c_void> {
    unsafe {
        let dos_header = &*(dll_base as *const IMAGE_DOS_HEADER);
        let nt_headers =
            &*((dll_base as usize + dos_header.e_lfanew as usize) as *const IMAGE_NT_HEADERS);
        let export_directory_rva = nt_headers.OptionalHeader.DataDirectory[0].VirtualAddress;
        let export_directory = &*((dll_base as usize + export_directory_rva as usize)
            as *const IMAGE_EXPORT_DIRECTORY);

        let names_rva = export_directory.AddressOfNames;
        let functions_rva = export_directory.AddressOfFunctions;
        let ordinals_rva = export_directory.AddressOfNameOrdinals;

        let names = std::slice::from_raw_parts(
            (dll_base as usize + names_rva as usize) as *const u32,
            export_directory.NumberOfNames as usize,
        );
        let ordinals = std::slice::from_raw_parts(
            (dll_base as usize + ordinals_rva as usize) as *const u16,
            export_directory.NumberOfNames as usize,
        );

        for i in 0..export_directory.NumberOfNames as usize {
            let name_ptr = (dll_base as usize + names[i] as usize) as *const u8;
            let name = std::ffi::CStr::from_ptr(name_ptr as *const i8)
                .to_str()
                .unwrap_or_default();
            if name == function_name {
                let ordinal = ordinals[i] as usize;
                let function_rva =
                    *((dll_base as usize + functions_rva as usize) as *const u32).add(ordinal);
                return Some((dll_base as usize + function_rva as usize) as *const c_void);
            }
        }
    }
    None
}

fn create_process(process_name: &str) -> winapi::shared::ntdef::HANDLE {
        // Create startup info struct
        let mut si = unsafe { std::mem::zeroed::<winapi::um::processthreadsapi::STARTUPINFOA>() };
        si.cb = std::mem::size_of::<winapi::um::processthreadsapi::STARTUPINFOA>() as u32;
    
        // Create process info struct
        let mut pi = unsafe { std::mem::zeroed::<winapi::um::processthreadsapi::PROCESS_INFORMATION>() };
    
        // Create process in suspended state
        let command_line_with_null: Vec<u8> = process_name.bytes().chain(std::iter::once(0)).collect();
        let success = unsafe {
            winapi::um::processthreadsapi::CreateProcessA(
                std::ptr::null_mut(),
                command_line_with_null.as_ptr() as *mut i8,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                winapi::um::winbase::CREATE_SUSPENDED,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut si,
                &mut pi
            )
        };
    
        if success == 0 {
            //return null handle
            return std::ptr::null_mut();
        }
    
        // Get the process handle
        let process_handle = pi.hProcess;

        //return the handle
        process_handle
}
