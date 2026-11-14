use clap::Subcommand;

/// Available QuantumVault CLI commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new secure vault.
    Init,

    /// Check the current vault status.
    Status,

    /// Mount the secure vault.
    Mount,

    /// Unmount the secure vault.
    Unmount,
}
