use clap::Subcommand;

/// Available QuantumVault commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Display the current vault status.
    Status,

    /// Initialize a new QuantumVault vault.
    Init,
}
