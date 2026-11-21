pub mod identity;
pub mod init;
pub mod key;
pub mod mount;
pub mod password;
pub mod status;

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init {
        #[arg(long, default_value = "vault")]
        path: PathBuf,
    },

    Password {
        #[command(subcommand)]
        command: PasswordCommands,
    },

    Status,

    Mount,

    Unmount,

    Key {
        #[command(subcommand)]
        command: KeyCommands,
    },

    Identity {
        #[command(subcommand)]
        command: IdentityCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum PasswordCommands {
    Set,
    Verify,
}

#[derive(Debug, Subcommand)]
pub enum KeyCommands {
    Generate {
        #[arg(long, default_value = "default-identity")]
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum IdentityCommands {
    List,
}
