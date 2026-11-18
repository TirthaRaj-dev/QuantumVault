pub mod init;
pub mod password;
pub mod status;

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new secure vault.
    Init {
        /// Directory where the vault will be created.
        #[arg(long, default_value = "vault")]
        path: PathBuf,
    },

    /// Manage the master password.
    Password {
        #[command(subcommand)]
        command: PasswordCommands,
    },

    /// Check the current vault status.
    Status,

    /// Mount the secure vault.
    Mount,

    /// Unmount the secure vault.
    Unmount,
}

#[derive(Debug, Subcommand)]
pub enum PasswordCommands {
    /// Set the master password for the vault.
    Set,
}
