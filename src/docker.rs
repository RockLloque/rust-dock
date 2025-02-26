use bollard::Docker;

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    fn new(path: Option<&str>) -> Self {
        // TODO: remove cast to String
        let path: String = if let Some(p) = path {
            p.to_string()
        } else {
            std::env::var("DOCKER_CONNECTION_SOCKET_PATH")
                .expect("Set a docker conenction path in .env")
        };

        let docker = Docker::connect_with_unix(path.as_str(), 120, bollard::API_DEFAULT_VERSION)
            .expect("Failed to connect to docker");
        Self { docker }
    }
}
