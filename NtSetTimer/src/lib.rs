#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;
use std::ptr::null_mut;
use std::time::Duration;
use windows::Win32::Foundation::{HANDLE, NTSTATUS, STATUS_SUCCESS};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::core::PCSTR;
use std::error::Error;
use std::fmt;

type NtCreateTimerFn = unsafe extern "system" fn(
    TimerHandle: *mut HANDLE,
    DesiredAccess: u32,
    ObjectAttributes: *mut c_void,
    TimerType: u32,
) -> NTSTATUS;

type NtSetTimerFn = unsafe extern "system" fn(
    TimerHandle: HANDLE,
    DueTime: *mut i64,
    TimerApcRoutine: Option<unsafe extern "system" fn(*mut c_void, u32, i32)>,
    TimerContext: *mut c_void,
    ResumeTimer: bool,
    Period: i32,
    PreviousState: *mut bool,
) -> NTSTATUS;

#[derive(Debug)]
pub struct TimerError {
    message: String,
}

impl Error for TimerError {}

impl fmt::Display for TimerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Creates and sets a timer that will fire after the specified duration
/// 
/// # Arguments
/// 
/// * `duration` - The duration after which the timer should fire
/// * `callback` - Optional callback function to be called when the timer fires
/// * `context` - Optional context to be passed to the callback function
/// 
/// # Returns
/// 
/// Returns a Result containing the timer handle if successful, or a TimerError if failed
pub fn set_timer(
    duration: Duration,
    callback: Option<unsafe extern "system" fn(*mut c_void, u32, i32)>,
    context: *mut c_void,
) -> Result<HANDLE, TimerError> {
    unsafe {
        // Get function pointers from ntdll.dll
        let ntdll = GetModuleHandleA(PCSTR::from_raw("ntdll.dll\0".as_ptr()))
            .map_err(|e| TimerError { message: format!("Failed to get ntdll.dll handle: {:?}", e) })?;

        let nt_create_timer: NtCreateTimerFn = std::mem::transmute(
            GetProcAddress(ntdll, PCSTR::from_raw("NtCreateTimer\0".as_ptr()))
                .ok_or_else(|| TimerError { message: "Failed to get NtCreateTimer address".to_string() })?
        );

        let nt_set_timer: NtSetTimerFn = std::mem::transmute(
            GetProcAddress(ntdll, PCSTR::from_raw("NtSetTimer\0".as_ptr()))
                .ok_or_else(|| TimerError { message: "Failed to get NtSetTimer address".to_string() })?
        );

        // Create a timer object
        let mut timer_handle = HANDLE::default();
        let status = nt_create_timer(
            &mut timer_handle,
            0x1F0003, // TIMER_ALL_ACCESS
            null_mut(), // No object attributes
            0, // Notification timer
        );

        if status != STATUS_SUCCESS {
            return Err(TimerError { message: format!("Failed to create timer: {:?}", status) });
        }

        // Convert duration to 100-nanosecond units (negative for relative time)
        let mut due_time: i64 = -((duration.as_nanos() / 100) as i64);

        let status = nt_set_timer(
            timer_handle,
            &mut due_time,
            callback,
            context,
            false, // Don't resume timer
            0, // No period
            null_mut() // Don't care about previous state
        );

        if status != STATUS_SUCCESS {
            return Err(TimerError { message: format!("Failed to set timer: {:?}", status) });
        }

        Ok(timer_handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_timer_creation() {
        let result = set_timer(Duration::from_secs(1), None, null_mut());
        assert!(result.is_ok());
    }
}
