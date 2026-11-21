use crate::integration::key_manager::KeyManager;

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

const VAULT_PATH: &str = "vault";
const IDENTITY_FILE: &str = "identities.txt";

/// Mock key manager for CLI integration testing.
///
/// Replace this implementation when Member 3 provides
/// the actual key-management module.
pub struct MockKeyManager {
    vault_path: std::path::PathBuf,
}

impl MockKeyManager {
    pub fn new(vault_path: &Path) -> Self {
        Self {
            vault_path: vault_path.to_path_buf(),
        }
    }

    fn identity_file(&self) -> std::path::PathBuf {
        self.vault_path.join(IDENTITY_FILE)
    }
}

impl KeyManager for MockKeyManager {
    fn initialize_identity(&self) -> io::Result<()> {
        if !self.vault_path.join("config.toml").is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Vault is not initialized.",
            ));
        }

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.identity_file())?;

        Ok(())
    }

    fn unlock(&self, _password: &[u8]) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Mock key manager does not implement cryptographic unlocking.",
        ))
    }

    fn lock(&self) -> io::Result<()> {
        Ok(())
    }

    fn generate_key(&self, identity_name: &str) -> io::Result<()> {
        if !self.vault_path.join("config.toml").is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Vault is not initialized. Run 'quantumvault init' first.",
            ));
        }

        if identity_name.trim().is_empty() || identity_name.contains(['\n', '\r']) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Identity name cannot be empty or contain line breaks.",
            ));
        }

        let identities = self.list_identities()?;

        if identities.iter().any(|name| name == identity_name) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "An identity with this name already exists.",
            ));
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.identity_file())?;

        writeln!(file, "{}", identity_name)?;
        file.sync_all()?;

        Ok(())
    }

    fn list_identities(&self) -> io::Result<Vec<String>> {
        let path = self.identity_file();

        if !path.exists() {
            return Ok(Vec::new());
        }

        let contents = fs::read_to_string(path)?;

        Ok(contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect())
    }
}

/// Check that the vault exists before using key commands.
fn ensure_vault_exists(vault_path: &Path) -> io::Result<()> {
    if !vault_path.join("config.toml").is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Vault is not initialized. Run 'quantumvault init' first.",
        ));
    }

    Ok(())
}

/// CLI command: quantumvault key generate
pub fn run_generate(identity_name: &str) -> io::Result<()> {
    let vault_path = Path::new(VAULT_PATH);

    println!("\nQuantumVault Key Generation\n");
    println!("Checking vault...");

    ensure_vault_exists(vault_path)?;

    let manager = MockKeyManager::new(vault_path);

    manager.generate_key(identity_name)?;

    println!("Mock key identity created successfully.");
    println!("Identity name: {}", identity_name);
    println!("NOTE: No cryptographic key was generated.");
    println!("Actual key generation requires Member 3's module.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_test_vault() -> std::path::PathBuf {
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!(
            "quantumvault_key_test_{}_{}",
            std::process::id(),
            unique_id
        ));

        fs::create_dir_all(path.join("data")).unwrap();
        fs::write(path.join("config.toml"), "# Test vault").unwrap();

        path
    }

    #[test]
    fn generates_mock_identity() {
        let vault = create_test_vault();
        let manager = MockKeyManager::new(&vault);

        assert!(manager.generate_key("test-identity").is_ok());

        let identities = manager.list_identities().unwrap();
        assert_eq!(identities, vec!["test-identity"]);

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn rejects_duplicate_identity() {
        let vault = create_test_vault();
        let manager = MockKeyManager::new(&vault);

        manager.generate_key("test-identity").unwrap();

        assert_eq!(
            manager.generate_key("test-identity").unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn rejects_empty_identity_name() {
        let vault = create_test_vault();
        let manager = MockKeyManager::new(&vault);

        assert!(manager.generate_key(" ").is_err());

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn lists_empty_identity_collection() {
        let vault = create_test_vault();
        let manager = MockKeyManager::new(&vault);

        assert!(manager.list_identities().unwrap().is_empty());

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn rejects_key_generation_without_initialized_vault() {
        let vault = create_test_vault();
        fs::remove_file(vault.join("config.toml")).unwrap();

        let manager = MockKeyManager::new(&vault);

        assert!(manager.generate_key("test-identity").is_err());

        fs::remove_dir_all(vault).unwrap();
    }
}
