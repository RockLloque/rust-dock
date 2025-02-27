mod cli;
mod docker;

use bollard::secret::{ContainerSummary, ImageSummary};
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
            ListCommands::Images { all } => {
                println!("Printing Images");
                match docker_client.list_images(all).await {
                    Err(e) => eprintln!("Error listing images: {}", e),
                    Ok(images) => {
                        for ImageSummary {
                            id,
                            parent_id,
                            containers,
                            ..
                        } in images
                        {
                            println!("{id}\t{parent_id}\t{containers}");
                        }
                    }
                }
            }
        },
        Command::Start { name } => {
            println!("Starting container: {name}");
            match docker_client.start_container(name.as_str()).await {
                Err(e) => eprintln!("Error starting container: {}", e),
                Ok(_) => println!("Started container {}", &name),
            }
        }
        Command::Stop { name, delay } => {
            println!("Stopping container: {name}");
            match docker_client.stop_container(name.as_str(), delay).await {
                Err(e) => eprintln!("Error stopping container: {}", e),
                Ok(_) => println!("Stopped container {}", &name),
            }
        }
    }
}
