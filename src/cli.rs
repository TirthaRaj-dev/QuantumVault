use clap::Parser;

use crate::commands::Commands;

/// QuantumVault command-line interface.
#[derive(Debug, Parser)]
#[command(
    name = "quantumvault",
    version,
    about = "QuantumVault CLI",
    long_about = "QuantumVault CLI\n\nA command-line interface for managing a secure local filesystem protected by post-quantum cryptography.",
    help_template = "\
{before-help}{name} {version}
{about-with-newline}
Usage: {usage}

Available Commands:
{subcommands}

Options:
{options}
{after-help}"
)]
pub struct Cli {
    /// Available QuantumVault commands.
    #[command(subcommand)]
    pub command: Commands,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::PasswordCommands;

    #[test]
    fn parses_status_command() {
        let cli = Cli::try_parse_from(["quantumvault", "status"]);

        assert!(matches!(cli.unwrap().command, Commands::Status));
    }

    #[test]
    fn parses_init_command() {
        let cli = Cli::try_parse_from(["quantumvault", "init"]);

        assert!(matches!(cli.unwrap().command, Commands::Init { .. }));
    }

    #[test]
    fn parses_custom_init_path() {
        let cli = Cli::try_parse_from(["quantumvault", "init", "--path", "test-vault"]).unwrap();

        match cli.command {
            Commands::Init { path } => {
                assert_eq!(path, std::path::PathBuf::from("test-vault"));
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[test]
    fn parses_mount_command() {
        let cli = Cli::try_parse_from(["quantumvault", "mount"]);

        assert!(matches!(cli.unwrap().command, Commands::Mount));
    }

    #[test]
    fn parses_unmount_command() {
        let cli = Cli::try_parse_from(["quantumvault", "unmount"]);

        assert!(matches!(cli.unwrap().command, Commands::Unmount));
    }

    #[test]
    fn rejects_unknown_command() {
        let cli = Cli::try_parse_from(["quantumvault", "unknown"]);

        assert!(cli.is_err());
    }

    #[test]
    fn parses_password_command() {
        let cli = Cli::try_parse_from(["quantumvault", "password", "set"]).unwrap();

        assert!(matches!(
            cli.command,
            Commands::Password {
                command: crate::commands::PasswordCommands::Set
            }
        ));
    }

    #[test]
    fn rejects_unknown_password_subcommand() {
        let cli = Cli::try_parse_from(["quantumvault", "password", "unknown"]);

        assert!(cli.is_err());
    }

    #[test]
    fn parses_password_verify_command() {
        let cli = Cli::try_parse_from(["quantumvault", "password", "verify"]).unwrap();

        assert!(matches!(
            cli.command,
            Commands::Password {
                command: PasswordCommands::Verify
            }
        ));
    }
}
