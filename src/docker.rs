use bollard::{
    Docker,
    container::{ListContainersOptions, StartContainerOptions, StopContainerOptions},
    errors::Error,
    image::{CreateImageOptions, ListImagesOptions},
    secret::{ContainerSummary, ImageSummary},
};
use futures_util::TryStreamExt;

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn new(path: Option<&str>) -> Self {
        // TODO: remove cast to String
        let path: String = if let Some(p) = path {
            p.to_string()
        } else {
            std::env::var("DOCKER_CONNECTION_SOCKET_PATH")
                .expect("Set a docker connection path in .env")
        };
        let docker = Docker::connect_with_unix(path.as_str(), 120, bollard::API_DEFAULT_VERSION)
            .expect("Failed to connect to docker");
        Self { docker }
    }
    pub async fn list_containers(&self, all: bool) -> Result<Vec<ContainerSummary>, Error> {
        let options = ListContainersOptions::<String> {
            all,
            ..Default::default()
        };

        let containers = self.docker.list_containers(Some(options)).await?;
        Ok(containers)
    }

    pub async fn list_images(&self, all: bool) -> Result<Vec<ImageSummary>, Error> {
        let options = ListImagesOptions::<String> {
            all,
            ..Default::default()
        };
        let images = self.docker.list_images(Some(options)).await?;
        Ok(images)
    }

    pub async fn start_container(&self, container_name: &str) -> Result<(), Error> {
        self.docker
            .start_container(container_name, None::<StartContainerOptions<String>>)
            .await?;
        Ok(())
    }

    pub async fn stop_container(&self, container_name: &str, delay: i64) -> Result<(), Error> {
        let options = StopContainerOptions { t: delay };

        self.docker
            .stop_container(container_name, Some(options))
            .await?;
        Ok(())
    }

    pub async fn create_image(&self, image_name: &str) -> Result<(), Error> {
        let options = CreateImageOptions {
            from_image: image_name,
            ..Default::default()
        };
        let mut stream = self.docker.create_image(Some(options), None, None);

        while let Some(msg) = stream.try_next().await? {
            if let Some(status) = msg.status {
                println!("{status}");
            }
        }
        Ok(())
    }
}
