mod lib;
use lib::set_timer;
use std::time::Duration;
use std::ptr::null_mut;
use windows::Win32::System::Threading::{WaitForSingleObject, INFINITE};

fn main() {
    println!("Starting NtSetTimer test...");

    // Set a timer for 2 seconds
    let timer_handle = set_timer(
        Duration::from_secs(2),
        None, // No callback
        null_mut() // No context
    ).expect("Failed to set timer");

    println!("Timer created and set successfully!");
    println!("Waiting for timer to fire...");

    // Wait for the timer to fire
    unsafe {
        let result = WaitForSingleObject(timer_handle, INFINITE);
        println!("Timer fired! Result: {:?}", result);
    }
}
