mod cli;
mod commands;
mod integration;
mod vault;

use clap::Parser;
use commands::{Commands, IdentityCommands, KeyCommands, PasswordCommands};

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            if let Err(error) = commands::init::run(path) {
                eprintln!("Error initializing vault: {error}");
            }
        }

        Commands::Password { command } => match command {
            PasswordCommands::Set => {
                if let Err(error) = commands::password::run() {
                    eprintln!("Error setting password: {error}");
                }
            }

            PasswordCommands::Verify => {
                if let Err(error) = commands::password::verify() {
                    eprintln!("Password verification failed: {error}");
                }
            }
        },

        Commands::Status => {
            commands::status::run();
        }

        Commands::Mount => {
            if let Err(error) = commands::mount::run_mount() {
                eprintln!("Error mounting vault: {error}");
            }
        }

        Commands::Unmount => {
            if let Err(error) = commands::mount::run_unmount() {
                eprintln!("Error unmounting vault: {error}");
            }
        }

        Commands::Key { command } => match command {
            KeyCommands::Generate { name } => {
                if let Err(error) = commands::key::run_generate(&name) {
                    eprintln!("Error generating identity: {error}");
                }
            }
        },

        Commands::Identity { command } => match command {
            IdentityCommands::List => {
                if let Err(error) = commands::identity::run_list() {
                    eprintln!("Error listing identities: {error}");
                }
            }
        },
    }
}
