use RtlQueryEnvironmentVariable_U::get_environment_variable;

fn main() {
    // Test getting a specific environment variable
    match get_environment_variable("OneDrive") {
        Ok(Some(value)) => println!("OneDrive: {}", value),
        Ok(None) => println!("OneDrive not found"),
        Err(e) => eprintln!("Error getting OneDrive: {}", e),
    }

    //check for USERNAME environment variable
    match get_environment_variable("USERNAME") {
        Ok(Some(value)) => println!("USERNAME: {}", value),
        Ok(None) => println!("USERNAME not found"),
        Err(e) => eprintln!("Error getting USERNAME: {}", e),
    }
    
}
