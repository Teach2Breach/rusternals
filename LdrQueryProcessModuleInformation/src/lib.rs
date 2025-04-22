#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::Result;
use ntapi::ntldr::{LdrQueryProcessModuleInformation, RTL_PROCESS_MODULES, RTL_PROCESS_MODULE_INFORMATION};
use std::io::{self, Write};

pub struct ModuleInfo {
    pub name: String,
    pub path: String,
    pub base_address: usize,
    pub size: usize,
}

pub fn query_process_modules() -> Result<Vec<ModuleInfo>> {
    // Start with a reasonable buffer size
    let mut buffer_size = std::mem::size_of::<RTL_PROCESS_MODULES>() as u32;
    let mut buffer = vec![0u8; buffer_size as usize];
    let mut required_size = 0u32;

    // Keep trying with larger buffers until we succeed
    loop {
        let status = unsafe {
            LdrQueryProcessModuleInformation(
                buffer.as_mut_ptr() as *mut RTL_PROCESS_MODULES,
                buffer_size,
                &mut required_size,
            )
        };

        if status == 0 {
            break;
        }

        // If we get STATUS_INFO_LENGTH_MISMATCH, we need a larger buffer
        if status as u32 == 0xC0000004 && required_size > buffer_size {
            buffer_size = required_size;
            buffer = vec![0u8; buffer_size as usize];
            continue;
        }

        // For any other error, log it and return empty
        let _ = writeln!(io::stderr(), "Error getting module information: 0x{:X}", status);
        return Ok(Vec::new());
    }

    // Parse the module information
    let mut modules = Vec::new();
    unsafe {
        let module_info = &*(buffer.as_ptr() as *const RTL_PROCESS_MODULES);
        let module_count = module_info.NumberOfModules as usize;
        
        if module_count == 0 {
            return Ok(modules);
        }

        // Calculate the actual size needed for the modules array
        let modules_size = std::mem::size_of::<RTL_PROCESS_MODULE_INFORMATION>() * module_count;
        let modules_ptr = module_info.Modules.as_ptr();
        
        // Ensure the pointer is not null and the size is valid
        if modules_ptr.is_null() || modules_size > isize::MAX as usize {
            return Ok(modules);
        }

        // Create a slice of RTL_PROCESS_MODULE_INFORMATION structures
        let module_array = std::slice::from_raw_parts(
            modules_ptr,
            module_count,
        );

        for module in module_array {
            let name = String::from_utf8_lossy(
                &module.FullPathName[..module.FullPathName.iter().position(|&x| x == 0).unwrap_or(256)]
            ).into_owned();
            
            modules.push(ModuleInfo {
                name: name.clone(),
                path: name,
                base_address: module.ImageBase as usize,
                size: module.ImageSize as usize,
            });
        }
    }

    Ok(modules)
}
