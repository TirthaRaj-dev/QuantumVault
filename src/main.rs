mod cli;
mod commands;
pub mod config;
pub mod integration;
mod vault;

use clap::Parser;
use cli::Cli;
use commands::{Commands, PasswordCommands};

const APP_NAME: &str = "QuantumVault";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_header() {
    println!("========================================");
    println!("          {} CLI", APP_NAME);
    println!("========================================");
    println!("Version: {}\n", VERSION);
}

fn run_command(command: Commands) {
    print_header();

    match command {
        Commands::Init { path } => {
            if let Err(error) = commands::init::run(path) {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        }

        Commands::Password { command } => match command {
            PasswordCommands::Set => {
                if let Err(error) = commands::password::run() {
                    eprintln!("Password setup failed: {}", error);
                    std::process::exit(1);
                }
            }

            PasswordCommands::Verify => {
                if let Err(error) = commands::password::verify() {
                    eprintln!("Password verification failed: {}", error);
                    std::process::exit(1);
                }
            }
        },

        Commands::Status => {
            commands::status::run();
        }

        Commands::Mount => {
            if let Err(error) = commands::mount::run_mount() {
                eprintln!("\nMount failed: {}", error);
                std::process::exit(1);
            }
        }

        Commands::Unmount => {
            if let Err(error) = commands::mount::run_unmount() {
                eprintln!("\nUnmount failed: {}", error);
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    run_command(cli.command);
}
