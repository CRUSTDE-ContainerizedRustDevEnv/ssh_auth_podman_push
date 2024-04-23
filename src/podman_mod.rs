// podman_ssh_auth/src/podman_mod.rs

use crate::secrets_always_local_mod::docker_hub_mod;

use crate::RED;
use crate::RESET;

/// push to docker hub retrieving the secret_token from encrypted file
///
/// If the secret_token is not stored ask user to interactively input the secret_token, encrypt and store.
pub fn push(image_url: &str) {
    // check if the plain-text file from `podman login` exists and warn the user because it is a security vulnerability.
    let file_auth = "${XDG_RUNTIME_DIR}/containers/auth.json";
    let xdg_runtime_dir = std::env::var_os("XDG_RUNTIME_DIR")
        .unwrap_or_else(|| panic!("{RED}Error: The env var XDG_RUNTIME_DIR does not exist. This OS cannot run podman.{RESET}"))
        .to_string_lossy()
        .to_string();
    let file_auth_expanded = file_auth.replace("${XDG_RUNTIME_DIR}", &xdg_runtime_dir);
    dbg!(&file_auth_expanded);
    let file_auth_expanded = camino::Utf8Path::new(&file_auth_expanded);

    if file_auth_expanded.exists() {
        panic!("{RED}Error: Security vulnerability: Found the file with plain-text secret_token: {file_auth_expanded} {RESET}")
    }

    // parse image_url to find the secret_token for registry and user_name
    // docker.io/bestiadev/crustde_cargo_img:cargo-xxx
    let mut splitted = image_url.split("/");
    let registry = splitted.next().unwrap();
    let user_name = splitted.next().unwrap();

    let docker_hub_client = docker_hub_mod::DockerHubClient::new_with_stored_secret_token(user_name, registry);
    docker_hub_client.push_to_docker_hub(image_url, user_name);
}
