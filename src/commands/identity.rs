use crate::commands::key::MockKeyManager;
use crate::integration::key_manager::KeyManager;

use std::io;
use std::path::Path;

const VAULT_PATH: &str = "vault";

/// CLI command: quantumvault identity list
pub fn run_list() -> io::Result<()> {
    let vault_path = Path::new(VAULT_PATH);

    if !vault_path.join("config.toml").is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Vault is not initialized. Run 'quantumvault init' first.",
        ));
    }

    println!("\nQuantumVault Identity List\n");

    let manager = MockKeyManager::new(vault_path);
    let identities = manager.list_identities()?;

    if identities.is_empty() {
        println!("No identities found.");
    } else {
        for (index, identity) in identities.iter().enumerate() {
            println!("{}. {}", index + 1, identity);
        }
    }

    println!("\nNOTE: These are mock identity records, not actual cryptographic keys.");

    Ok(())
}
