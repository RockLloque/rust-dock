mod cli;
mod docker;

use bollard::secret::ContainerSummary;
use clap::Parser;
use cli::{Cli, Command, ListCommands};
use docker::DockerClient;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Load the .env file
    dotenv().ok();
    // Parse the cli input
    let args = Cli::parse();
    let docker_client = DockerClient::new(None);

    // handle the commands
    match args.command {
        Command::List { list_command } => match list_command {
            ListCommands::Containers { all } => {
                println!("Printing Containers");
                match docker_client.list_containers(all).await {
                    Ok(containers) => {
                        for ContainerSummary {
                            id, names, status, ..
                        } in containers
                        {
                            println!(
                                "{}\t{}\t{}",
                                id.unwrap_or_default(),
                                names.unwrap_or_default().join(","),
                                status.unwrap_or_default()
                            )
                        }
                    }
                    Err(e) => eprintln!("Error listing containers: {}", e),
                }
            }
        },
    }
}
