use RtlQueryEnvironmentVariable_U::{get_environment_variable, get_uuid};

fn main() {
    // Test getting a specific environment variable
    match get_environment_variable("OneDrive") {
        Ok(Some(value)) => println!("OneDrive: {}", value),
        Ok(None) => println!("OneDrive not found"),
        Err(e) => eprintln!("Error getting OneDrive: {}", e),
    }

    // Test getting the UUID
    match get_uuid() {
        Ok(Some(uuid)) => println!("UUID: {}", uuid),
        Ok(None) => println!("UUID not found"),
        Err(e) => eprintln!("Error getting UUID: {}", e),
    }
}
