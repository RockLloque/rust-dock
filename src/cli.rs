use clap::{Parser, Subcommand};
/// Top-level Cli structure
#[derive(Parser)]
#[command(name = "mydocker")]
#[command(about = "A minimal Docker CLI in Rust")]
pub struct Cli {
    /// The main command to execute
    #[command(subcommand)]
    pub command: Command,
}

/// Enum for top-level commands
#[derive(Subcommand)]
pub enum Command {
    /// List resources
    List {
        /// Subcommands for listing resources
        #[command(subcommand)]
        list_command: ListCommands,
    },
    /// Start container
    Start {
        /// The container name
        name: String,
    },
    /// Stop container
    Stop {
        /// The container name
        name: String,
        /// Time delay before stop
        #[arg(short, long, default_value_t = 0)]
        delay: i64,
    },
}

/// Enum for subcommands under List
#[derive(Subcommand)]
pub enum ListCommands {
    /// List containers
    Containers {
        /// Include stopped containers
        #[arg(short, long)]
        all: bool,
    },
    /// List Images
    Images {
        /// Include stopped images
        #[arg(short, long)]
        all: bool,
    },
}
