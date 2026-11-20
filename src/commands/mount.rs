use crate::integration::filesystem::VaultFilesystem;

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const VAULT_PATH: &str = "vault";
const MOUNT_POINT: &str = "vault/mount";
const MOUNT_STATE_FILE: &str = ".mock-mounted";

/// Mock filesystem backend.
///
/// This simulates mounting until Member 2 provides
/// the actual FUSE implementation.
pub struct MockVaultFilesystem {
    state_file: PathBuf,
}

impl MockVaultFilesystem {
    pub fn new(vault_path: &Path) -> Self {
        Self {
            state_file: vault_path.join(MOUNT_STATE_FILE),
        }
    }
}

impl VaultFilesystem for MockVaultFilesystem {
    fn mount(&self, mount_point: &Path) -> io::Result<()> {
        if !mount_point.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Mount point must be an existing directory.",
            ));
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.state_file)?;

        let mount_path = mount_point.canonicalize()?;

        if let Err(error) = file
            .write_all(mount_path.to_string_lossy().as_bytes())
            .and_then(|_| file.sync_all())
        {
            drop(file);
            let _ = fs::remove_file(&self.state_file);
            return Err(error);
        }

        println!("Mock filesystem: mount operation simulated.");
        Ok(())
    }

    fn unmount(&self, mount_point: &Path) -> io::Result<()> {
        let stored_path = fs::read_to_string(&self.state_file)?;

        let requested_path = mount_point.canonicalize()?;

        if stored_path != requested_path.to_string_lossy() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The supplied mount point does not match the mounted vault.",
            ));
        }

        fs::remove_file(&self.state_file)?;

        println!("Mock filesystem: unmount operation simulated.");
        Ok(())
    }
}

/// Check that the vault is initialized and its password is configured.
fn check_vault(vault_path: &Path) -> io::Result<()> {
    if !vault_path.join("config.toml").is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Vault is not initialized. Run 'quantumvault init' first.",
        ));
    }

    if !vault_path.join("password.hash").is_file() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Master password is not configured. Run 'quantumvault password set'.",
        ));
    }

    Ok(())
}

/// Validate and prepare the mount point.
fn prepare_mount_point(mount_point: &Path) -> io::Result<()> {
    if mount_point.exists() {
        if !mount_point.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Mount point exists but is not a directory.",
            ));
        }
    } else {
        fs::create_dir_all(mount_point)?;
    }

    Ok(())
}

/// Perform a mount using the supplied filesystem backend.
fn mount_with_backend(
    vault_path: &Path,
    mount_point: &Path,
    filesystem: &impl VaultFilesystem,
) -> io::Result<()> {
    check_vault(vault_path)?;
    prepare_mount_point(mount_point)?;
    filesystem.mount(mount_point)
}

/// Perform an unmount using the supplied filesystem backend.
fn unmount_with_backend(
    vault_path: &Path,
    mount_point: &Path,
    filesystem: &impl VaultFilesystem,
) -> io::Result<()> {
    if !vault_path.join("config.toml").is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Vault is not initialized.",
        ));
    }

    filesystem.unmount(mount_point)
}

/// CLI mount command.
pub fn run_mount() -> io::Result<()> {
    let vault_path = Path::new(VAULT_PATH);
    let mount_point = Path::new(MOUNT_POINT);

    println!("\n========================================");
    println!("          QuantumVault Mount");
    println!("========================================");

    println!("\nChecking vault...");
    check_vault(vault_path)?;

    println!("Vault initialized.");

    // Authentication is required in this process.
    crate::commands::password::verify_for_mount()?;

    let filesystem = MockVaultFilesystem::new(vault_path);

    mount_with_backend(vault_path, mount_point, &filesystem)?;

    println!("\nVault mounted successfully.");
    println!("Mount point: {}", mount_point.display());
    println!("NOTE: Mock mode only. Actual FUSE mounting is pending.");

    Ok(())
}

/// CLI unmount command.
pub fn run_unmount() -> io::Result<()> {
    let vault_path = Path::new(VAULT_PATH);
    let mount_point = Path::new(MOUNT_POINT);

    println!("\n========================================");
    println!("         QuantumVault Unmount");
    println!("========================================");

    println!("\nUnmounting vault...");

    let filesystem = MockVaultFilesystem::new(vault_path);

    unmount_with_backend(vault_path, mount_point, &filesystem)?;

    println!("\nVault unmounted successfully.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_test_vault() -> PathBuf {
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!(
            "quantumvault_mount_test_{}_{}",
            std::process::id(),
            unique_id
        ));

        fs::create_dir_all(path.join("data")).unwrap();
        fs::write(path.join("config.toml"), "# Test vault").unwrap();
        fs::write(path.join("password.hash"), "$argon2id$test").unwrap();

        path
    }

    #[test]
    fn successfully_mounts_vault() {
        let vault = create_test_vault();
        let mount_point = vault.join("mount");
        fs::create_dir_all(&mount_point).unwrap();

        let filesystem = MockVaultFilesystem::new(&vault);

        assert!(mount_with_backend(&vault, &mount_point, &filesystem).is_ok());

        assert!(vault.join(MOUNT_STATE_FILE).is_file());

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn prevents_mounting_twice() {
        let vault = create_test_vault();
        let mount_point = vault.join("mount");
        fs::create_dir_all(&mount_point).unwrap();

        let filesystem = MockVaultFilesystem::new(&vault);

        mount_with_backend(&vault, &mount_point, &filesystem).unwrap();

        assert!(mount_with_backend(&vault, &mount_point, &filesystem).is_err());

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn rejects_uninitialized_vault() {
        let vault = create_test_vault();
        fs::remove_file(vault.join("config.toml")).unwrap();

        let mount_point = vault.join("mount");
        let filesystem = MockVaultFilesystem::new(&vault);

        assert!(mount_with_backend(&vault, &mount_point, &filesystem).is_err());

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn rejects_invalid_mount_point() {
        let vault = create_test_vault();
        let mount_point = vault.join("not_a_directory");
        fs::write(&mount_point, "not a directory").unwrap();

        let filesystem = MockVaultFilesystem::new(&vault);

        assert!(mount_with_backend(&vault, &mount_point, &filesystem).is_err());

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn successfully_unmounts_vault() {
        let vault = create_test_vault();
        let mount_point = vault.join("mount");
        fs::create_dir_all(&mount_point).unwrap();

        let filesystem = MockVaultFilesystem::new(&vault);

        mount_with_backend(&vault, &mount_point, &filesystem).unwrap();

        assert!(unmount_with_backend(&vault, &mount_point, &filesystem).is_ok());

        assert!(!vault.join(MOUNT_STATE_FILE).exists());

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn rejects_unmount_when_not_mounted() {
        let vault = create_test_vault();
        let mount_point = vault.join("mount");
        fs::create_dir_all(&mount_point).unwrap();

        let filesystem = MockVaultFilesystem::new(&vault);

        assert!(unmount_with_backend(&vault, &mount_point, &filesystem).is_err());

        fs::remove_dir_all(vault).unwrap();
    }

    #[test]
    fn rejects_wrong_unmount_point() {
        let vault = create_test_vault();
        let mount_point = vault.join("mount");
        let wrong_mount_point = vault.join("wrong-mount");

        fs::create_dir_all(&mount_point).unwrap();
        fs::create_dir_all(&wrong_mount_point).unwrap();

        let filesystem = MockVaultFilesystem::new(&vault);

        mount_with_backend(&vault, &mount_point, &filesystem).unwrap();

        assert!(unmount_with_backend(&vault, &wrong_mount_point, &filesystem).is_err());

        fs::remove_dir_all(vault).unwrap();
    }
}
