pub mod init;
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
}

#[derive(Debug, Subcommand)]
pub enum PasswordCommands {
    Set,
    Verify,
}
