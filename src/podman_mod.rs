// podman_ssh_auth/src/podman_mod.rs

use crate::secrets_always_local_mod::docker_hub_mod;

/// store the docker hub token securely encrypted with a SSH private key
pub fn login(user_name: &str, registry: &str) {
    log::info!("start login()");
    // save token for registry and username
    let _docker_hub_client = docker_hub_mod::DockerHubClient::new_with_stored_token(user_name, registry);
}

/// push to docker hub retrieving the token from encrypted file
pub fn push(image_url: &str) {
    log::info!("start push()");
    // parse image_url to find the token for registry and user_name
    // docker.io/bestiadev/crustde_cargo_img:cargo-xxx
    let mut splitted = image_url.split("/");
    let registry = splitted.next().unwrap();
    let user_name = splitted.next().unwrap();

    let docker_hub_client = docker_hub_mod::DockerHubClient::new_with_stored_token(user_name, registry);
    docker_hub_client.push_to_docker_hub(image_url, user_name);
}
