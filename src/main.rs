mod cli;
mod commands;
mod vault;

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
        Commands::Init { path } => {
            print_header();
            println!("\nInitializing QuantumVault...\n");

            // path is already a PathBuf.
            match vault::initialize(&path) {
                Ok(()) => {
                    println!("\nQuantumVault initialized successfully!");
                    println!("Vault location: {}", path.display());
                }
                Err(error) => {
                    eprintln!("\nVault initialization failed: {}", error);
                    std::process::exit(1);
                }
            }
        }

        Commands::Status => {
            print_header();
            println!("\nVault Status");
            println!("------------");
            println!("Application   : {}", APP_NAME);
            println!("Version       : {}", VERSION);
            println!("Note          : Persistent status detection is not yet implemented.");
        }

        Commands::Mount => {
            print_header();
            println!("\nMount Interface");
            println!("---------------");
            println!("Mounting will be implemented during filesystem integration.");
        }

        Commands::Unmount => {
            print_header();
            println!("\nUnmount Interface");
            println!("-----------------");
            println!("Unmounting will be implemented during filesystem integration.");
        }
    }
}

fn main() {
    let cli = Cli::parse();
    run_command(cli.command);
}
