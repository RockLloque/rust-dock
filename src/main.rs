mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommands};

fn main() {
    // Parse the cli input
    let args = Cli::parse();

    // handle the commands
    match args {
        Command::List { list_command } => match list_command {
            ListCommands::Containers => {
                println!("Listing all containers");
            }
        },
    }
}
