mod cli;
mod commands;

use clap::Parser;
use cli::Cli;
use commands::Commands;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            println!("QuantumVault status command selected.");
            println!("Status functionality will be implemented on Day 3.");
        }

        Commands::Init => {
            println!("QuantumVault init command selected.");
            println!("Vault initialization will be implemented on Day 4.");
        }
    }
}
