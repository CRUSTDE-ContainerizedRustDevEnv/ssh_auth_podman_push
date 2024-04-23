// podman_ssh_auth/src/podman_mod.rs

use crate::secrets_always_local_mod::docker_hub_mod;

/// push to docker hub retrieving the secret_token from encrypted file
///
/// If the secret_token is not stored ask user to interactively input the secret_token, encrypt and store.
pub fn push(image_url: &str) {
    // parse image_url to find the secret_token for registry and user_name
    // docker.io/bestiadev/crustde_cargo_img:cargo-xxx
    let mut splitted = image_url.split("/");
    let registry = splitted.next().unwrap();
    let user_name = splitted.next().unwrap();

    let docker_hub_client = docker_hub_mod::DockerHubClient::new_with_stored_secret_token(user_name, registry);
    docker_hub_client.push_to_docker_hub(image_url, user_name);
}
