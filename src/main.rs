mod cli;
mod commands;

use clap::Parser;
use cli::Cli;
use commands::Commands;

const APP_NAME: &str = "QuantumVault";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_header() {
    println!("========================================");
    println!("          {} CLI", APP_NAME);
    println!("========================================");
}

fn run_command(command: Commands) {
    match command {
        Commands::Init => {
            print_header();
            println!("\nInitialization");
            println!("--------------");
            println!("Command       : init");
            println!("Purpose       : Initialize a new secure vault");
            println!("Implementation: Scheduled for Day 4");
        }

        Commands::Status => {
            print_header();
            println!("\nVault Status");
            println!("------------");
            println!("Application   : {}", APP_NAME);
            println!("Version       : {}", VERSION);
            println!("Vault status  : Not yet initialized");
            println!(
                "Note          : Full vault status checking will be implemented with vault initialization."
            );
        }

        Commands::Mount => {
            print_header();
            println!("\nMount Interface");
            println!("---------------");
            println!("Command       : mount");
            println!("Purpose       : Mount the secure vault");
            println!("Implementation: Planned for filesystem integration.");
        }

        Commands::Unmount => {
            print_header();
            println!("\nUnmount Interface");
            println!("-----------------");
            println!("Command       : unmount");
            println!("Purpose       : Unmount the secure vault");
            println!("Implementation: Planned for filesystem integration.");
        }
    }
}

fn main() {
    let cli = Cli::parse();
    run_command(cli.command);
}
