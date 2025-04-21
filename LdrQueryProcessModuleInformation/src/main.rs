mod lib;
use lib::query_process_modules;
fn main() {
    match query_process_modules() {
        Ok(modules) => {
            println!("Loaded modules:");
            for (i, module) in modules.iter().enumerate() {
                println!("\nModule {}:", i + 1);
                println!("  Name: {}", module.name);
                println!("  Path: {}", module.path);
                println!("  Base Address: 0x{:X}", module.base_address);
                println!("  Size: {} bytes", module.size);
            }
        }
        Err(e) => {
            eprintln!("Error querying process modules: {}", e);
        }
    }
}
