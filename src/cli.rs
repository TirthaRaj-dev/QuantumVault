use clap::Parser;

use crate::commands::Commands;

/// QuantumVault - Post-Quantum Secure Local Filesystem CLI.
#[derive(Debug, Parser)]
#[command(
    name = "quantumvault",
    version,
    about = "QuantumVault: Post-Quantum Secure Local Filesystem",
    long_about = "QuantumVault provides a command-line interface for managing a secure local filesystem protected by post-quantum cryptography."
)]
pub struct Cli {
    /// Available commands.
    #[command(subcommand)]
    pub command: Commands,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_status_command() {
        let cli = Cli::try_parse_from(["quantumvault", "status"]);

        assert!(cli.is_ok());
        assert!(matches!(cli.unwrap().command, Commands::Status));
    }

    #[test]
    fn parses_init_command() {
        let cli = Cli::try_parse_from(["quantumvault", "init"]);

        assert!(cli.is_ok());
        assert!(matches!(cli.unwrap().command, Commands::Init));
    }

    #[test]
    fn rejects_unknown_command() {
        let cli = Cli::try_parse_from(["quantumvault", "unknown"]);

        assert!(cli.is_err());
    }
}
