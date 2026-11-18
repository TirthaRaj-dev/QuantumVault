use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand_core::OsRng;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use zeroize::Zeroizing;

const VAULT_PATH: &str = "vault";
const PASSWORD_FILE: &str = "password.hash";
const MIN_PASSWORD_LENGTH: usize = 8;

/// Runs the master password setup workflow.
pub fn run() -> Result<(), Box<dyn Error>> {
    let vault_path = Path::new(VAULT_PATH);

    // Make sure the vault has already been initialized.
    if !vault_path.join("config.toml").is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Vault not initialized. Run 'quantumvault init' first.",
        )
        .into());
    }

    let password_path = vault_path.join(PASSWORD_FILE);

    // Do not replace an existing master password.
    if password_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "A master password is already configured.",
        )
        .into());
    }

    println!("\nQuantumVault Password Setup\n");

    // Hidden terminal input.
    let password = Zeroizing::new(rpassword::prompt_password("Enter master password: ")?);

    let confirmation = Zeroizing::new(rpassword::prompt_password("Confirm master password: ")?);

    validate_password(&password, &confirmation)?;

    // Generate a random salt and create an Argon2id hash.
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|error| io::Error::other(error.to_string()))?
        .to_string();

    // create_new prevents overwriting an existing password file.
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&password_path)?;

    if let Err(error) = file
        .write_all(password_hash.as_bytes())
        .and_then(|_| file.sync_all())
    {
        drop(file);

        if let Err(cleanup_error) = std::fs::remove_file(&password_path) {
            eprintln!(
                "Warning: could not remove incomplete password file: {}",
                cleanup_error
            );
        }

        return Err(error.into());
    }

    println!("\nMaster password configured successfully.");

    Ok(())
}

/// Validates password input before hashing.
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

    #[test]
    fn accepts_valid_password() {
        assert!(validate_password("Quantum123!", "Quantum123!").is_ok());
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
        assert!(validate_password("Quantum123!", "Different123!").is_err());
    }

    #[test]
    fn generates_argon2id_password_hash() {
        let password = "Quantum123!";
        let salt = SaltString::generate(&mut OsRng);

        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        assert!(hash.starts_with("$argon2id$"));
        assert!(!hash.contains(password));
    }
}
