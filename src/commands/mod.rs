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

    /// Check the current vault status.
    Status,

    /// Mount the secure vault.
    Mount,

    /// Unmount the secure vault.
    Unmount,
}
