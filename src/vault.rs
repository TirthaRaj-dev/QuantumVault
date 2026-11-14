use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Initializes a new QuantumVault directory.
///
/// Refuses to overwrite an existing directory.
/// Removes the incomplete vault if initialization fails.
pub fn initialize(path: &Path) -> io::Result<()> {
    // Fails if the target directory already exists.
    // This protects existing vault data.
    fs::create_dir(path)?;

    println!("Creating vault directory...");

    let result = create_vault_contents(path);

    if let Err(error) = result {
        // Remove only the directory created by this operation.
        if let Err(cleanup_error) = fs::remove_dir_all(path) {
            eprintln!(
                "Warning: could not remove incomplete vault: {}",
                cleanup_error
            );
        }

        return Err(error);
    }

    Ok(())
}

fn create_vault_contents(path: &Path) -> io::Result<()> {
    // Create the directory for future encrypted files.
    let data_dir = path.join("data");
    fs::create_dir(&data_dir)?;

    println!("Creating configuration file...");

    let config_path = path.join("config.toml");

    // create_new prevents accidental replacement of a file.
    let mut config_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config_path)?;

    let config = r#"# QuantumVault configuration
# Initial development configuration.
# Security settings will be integrated with the
# key-management and cryptography modules.

[vault]
application = "QuantumVault"
format_version = 1
vault_name = "QuantumVault"
encryption_status = "not_configured"
key_management_status = "pending"
"#;

    config_file.write_all(config.as_bytes())?;
    config_file.sync_all()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn test_path(name: &str) -> std::path::PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);

        std::env::temp_dir().join(format!(
            "quantumvault-test-{}-{}-{}",
            name,
            std::process::id(),
            id
        ))
    }

    #[test]
    fn initializes_vault_directory_and_files() {
        let path = test_path("success");

        initialize(&path).expect("initialization should succeed");

        assert!(path.is_dir());
        assert!(path.join("data").is_dir());
        assert!(path.join("config.toml").is_file());

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn does_not_overwrite_existing_vault() {
        let path = test_path("existing");

        fs::create_dir_all(&path).unwrap();

        let existing_file = path.join("important.txt");
        fs::write(&existing_file, "keep this data").unwrap();

        let result = initialize(&path);

        assert!(result.is_err());
        assert_eq!(
            fs::read_to_string(&existing_file).unwrap(),
            "keep this data"
        );

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn rejects_invalid_parent_path() {
        let path = test_path("missing-parent").join("vault");

        let result = initialize(&path);

        assert!(result.is_err());
        assert!(!path.exists());
    }

    #[test]
    fn configuration_does_not_contain_master_password() {
        let path = test_path("config");

        initialize(&path).unwrap();

        let config = fs::read_to_string(path.join("config.toml")).unwrap();

        assert!(config.contains("application = \"QuantumVault\""));
        assert!(config.contains("format_version = 1"));
        assert!(!config.to_lowercase().contains("master_password"));

        fs::remove_dir_all(path).unwrap();
    }
}
