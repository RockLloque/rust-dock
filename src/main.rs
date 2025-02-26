mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommands};

// #[tokio::main]
fn main() {
    // Parse the cli input
    let args = Cli::parse();

    // handle the commands
    match args.command {
        Command::List { list_command } => match list_command {
            ListCommands::Containers { all } => {
                if all {
                    println!("Listing all containers");
                } else {
                    println!("Listing only running containers");
                }
            }
        },
    }
}
