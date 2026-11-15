
use std::io;
use std::path::PathBuf;

pub fn run(path: PathBuf) -> io::Result<()> {
    println!("Initializing QuantumVault...\n");

    match crate::vault::initialize(&path) {
        Ok(()) => {
            println!("\nQuantumVault initialized successfully!");
            println!("Vault location: {}", path.display());
            Ok(())
        }
        Err(error) => {
            eprintln!("\nVault initialization failed: {}", error);
            Err(error)
        }
    }
}
