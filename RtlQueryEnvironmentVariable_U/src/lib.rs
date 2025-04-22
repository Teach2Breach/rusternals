#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::core::Result;
use ntapi::ntrtl::RtlQueryEnvironmentVariable_U;
use winapi::shared::ntdef::UNICODE_STRING;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub fn get_environment_variable(name: &str) -> Result<Option<String>> {
    // Convert the variable name to a UNICODE_STRING
    let name_wide: Vec<u16> = name.encode_utf16().collect();
    let mut name_unicode = UNICODE_STRING {
        Length: (name_wide.len() * 2) as u16,
        MaximumLength: (name_wide.len() * 2) as u16,
        Buffer: name_wide.as_ptr() as *mut u16,
    };

    // Create a buffer for the result
    let mut value_buffer = vec![0u16; 1024];
    let mut value_unicode = UNICODE_STRING {
        Length: 0,
        MaximumLength: (value_buffer.len() * 2) as u16,
        Buffer: value_buffer.as_mut_ptr(),
    };

    // Query the environment variable
    let status = unsafe {
        RtlQueryEnvironmentVariable_U(
            std::ptr::null_mut(), // Use current environment
            &mut name_unicode,
            &mut value_unicode,
        )
    };

    if status == 0 {
        // Success - convert the result to a String
        let value = unsafe {
            OsString::from_wide(
                std::slice::from_raw_parts(
                    value_unicode.Buffer,
                    value_unicode.Length as usize / 2
                )
            )
        };
        Ok(Some(value.to_string_lossy().into_owned()))
    } else {
        // Variable not found or other error
        Ok(None)
    }
}