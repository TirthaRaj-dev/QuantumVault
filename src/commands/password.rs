use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand_core::OsRng;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use zeroize::Zeroizing;

const VAULT_PATH: &str = "vault";
const PASSWORD_FILE: &str = "password.hash";
const MIN_PASSWORD_LENGTH: usize = 8;

/// Interactive master password setup.
pub fn run() -> Result<(), Box<dyn Error>> {
    let vault_path = Path::new(VAULT_PATH);
    ensure_vault_exists(vault_path)?;

    println!("\nQuantumVault Password Setup\n");

    let password = Zeroizing::new(rpassword::prompt_password("Enter master password: ")?);

    let confirmation = Zeroizing::new(rpassword::prompt_password("Confirm master password: ")?);

    configure_password(vault_path, &password, &confirmation)?;

    println!("\nMaster password configured successfully.");
    Ok(())
}

/// Interactive master password verification.
pub fn verify() -> Result<(), Box<dyn Error>> {
    verify_for_mount()?;
    println!("\nMaster password verified successfully.");
    Ok(())
}

/// Verify the master password before allowing a mount.
pub fn verify_for_mount() -> io::Result<()> {
    let vault_path = Path::new(VAULT_PATH);
    ensure_vault_exists(vault_path)?;
    println!("\nChecking authentication...");

    let password = Zeroizing::new(
        rpassword::prompt_password("Enter master password: ")
            .map_err(|error| io::Error::other(error.to_string()))?,
    );

    if verify_password(vault_path, &password)? {
        println!("Authentication successful.");
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Incorrect master password.",
        ))
    }
}

/// Check that the vault has been initialized.
fn ensure_vault_exists(vault_path: &Path) -> io::Result<()> {
    if !vault_path.join("config.toml").is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Vault not initialized. Run 'quantumvault init' first.",
        ));
    }

    Ok(())
}

/// Validate and securely store a new master password.
fn configure_password(vault_path: &Path, password: &str, confirmation: &str) -> io::Result<()> {
    ensure_vault_exists(vault_path)?;
    validate_password(password, confirmation)?;

    let password_path = vault_path.join(PASSWORD_FILE);

    // create_new prevents accidental replacement, including concurrent attempts.
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|error| io::Error::other(error.to_string()))?
        .to_string();

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&password_path)?;

    if let Err(error) = file
        .write_all(password_hash.as_bytes())
        .and_then(|_| file.sync_all())
    {
        drop(file);

        if let Err(cleanup_error) = fs::remove_file(&password_path) {
            eprintln!(
                "Warning: could not remove incomplete password file: {}",
                cleanup_error
            );
        }

        return Err(error);
    }

    Ok(())
}

/// Verify a password against the stored Argon2id hash.
fn verify_password(vault_path: &Path, password: &str) -> io::Result<bool> {
    ensure_vault_exists(vault_path)?;

    let password_path = vault_path.join(PASSWORD_FILE);

    if !password_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Master password is not configured. Run 'quantumvault password set'.",
        ));
    }

    let stored_hash = fs::read_to_string(password_path)?;

    let parsed_hash = PasswordHash::new(&stored_hash).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Stored password hash is invalid: {}", error),
        )
    })?;

    // Argon2 performs the password-hash verification.
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Validate password and confirmation before hashing.
fn validate_password(password: &str, confirmation: &str) -> io::Result<()> {
    if password.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Password cannot be empty.",
        ));
    }

    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Password must contain at least 8 bytes.",
        ));
    }

    if password != confirmation {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Password confirmation does not match.",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    fn create_test_vault() -> PathBuf {
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!(
            "quantumvault_test_{}_{}",
            std::process::id(),
            unique_id
        ));

        fs::create_dir_all(path.join("data")).unwrap();
        fs::write(path.join("config.toml"), "# Test vault").unwrap();

        path
    }

    fn remove_test_vault(path: &Path) {
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn accepts_valid_password() {
        assert!(validate_password("StrongPass123", "StrongPass123").is_ok());
    }

    #[test]
    fn rejects_empty_password() {
        assert!(validate_password("", "").is_err());
    }

    #[test]
    fn rejects_short_password() {
        assert!(validate_password("short", "short").is_err());
    }

    #[test]
    fn rejects_password_mismatch() {
        assert!(validate_password("StrongPass123", "DifferentPass123").is_err());
    }

    #[test]
    fn sets_and_verifies_correct_password() {
        let vault = create_test_vault();

        assert!(configure_password(&vault, "StrongPass123", "StrongPass123").is_ok());

        assert!(verify_password(&vault, "StrongPass123").unwrap());

        remove_test_vault(&vault);
    }

    #[test]
    fn rejects_incorrect_password() {
        let vault = create_test_vault();

        configure_password(&vault, "StrongPass123", "StrongPass123").unwrap();

        assert!(!verify_password(&vault, "WrongPassword123").unwrap());

        remove_test_vault(&vault);
    }

    #[test]
    fn prevents_password_replacement() {
        let vault = create_test_vault();

        configure_password(&vault, "StrongPass123", "StrongPass123").unwrap();

        let second_attempt = configure_password(&vault, "AnotherPass123", "AnotherPass123");

        assert!(second_attempt.is_err());

        // The original password must still work.
        assert!(verify_password(&vault, "StrongPass123").unwrap());

        remove_test_vault(&vault);
    }

    #[test]
    fn generates_argon2id_hash_with_salt() {
        let vault = create_test_vault();

        configure_password(&vault, "StrongPass123", "StrongPass123").unwrap();

        let stored_hash = fs::read_to_string(vault.join(PASSWORD_FILE)).unwrap();

        assert!(stored_hash.starts_with("$argon2id$"));
        assert!(!stored_hash.contains("StrongPass123"));

        remove_test_vault(&vault);
    }
}
