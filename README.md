[//]: # (auto_md_to_doc_comments segment start A)

# podman_ssh_auth

[//]: # (auto_cargo_toml_to_md start)

**Store and use encrypted docker-hub token with SSH key**  
***version: 0.0.27 date: 2024-04-23 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/CRUSTDE-ContainerizedRustDevEnv/podman_ssh_auth)***

 ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
 ![rustlang](https://img.shields.io/badge/rustlang-orange)
 ![docker-hub](https://img.shields.io/badge/docker_hub-orange)

[//]: # (auto_cargo_toml_to_md end)

  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/podman_ssh_auth/blob/main/LICENSE)
  [![Rust](https://github.com/CRUSTDE-ContainerizedRustDevEnv/podman_ssh_auth/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/podman_ssh_auth/)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-396-green.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/podman_ssh_auth/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-299-blue.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/podman_ssh_auth/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-62-purple.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/podman_ssh_auth/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/podman_ssh_auth/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-183-orange.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/podman_ssh_auth/)

[//]: # (auto_lines_of_code end)

Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
I recommend using the [CRUSTDE - Containerized Rust Development Environment](https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod) to write Rust projects on Linux, isolated from your system.  

## Motivation

To access docker-hub you need a username+password or an access token.  
IMPORTANT: Treat access tokens like your password and keep them secret. Store your tokens securely in a credential manager for example.  
Access tokens are impossible to remember for an average human. We need to store them somewhere.  
FYI: Podman is an alternative "drop-in replacement" for Docker.  
I am sure they both store the docker-hub token for login with the command:

```bash
podman login --username user_name docker.io
docker login --username user_name docker.io
```



WARNING: Be aware that they store the token in "plain-text" in the file: `${XDG_RUNTIME_DIR}/containers/auth.json`.  
Ok, it is not really plain-text, but base64 encoding is not a security feature.  
This means that every attacker that can get to this well-known file, can log in to our Docker Hub account. No bueno!!!

I want to secure this token with encryption with an SSH key.  
We have already a lot of experience creating, managing and securing our SSH keys. The private key is secured by a passphrase we can remember and type. Every use of the token will need user interaction to type the passphrase. Very secure.  

If we are very self-confident in our current session, we can store the SSH key in ssh-agent and write our passphrase only once.  
WARNING: a dedicated attacker could read from ssh-agent and discover the access token without our user interaction. Use this at your discretion.  

## Development details

Read the development details in a separate md file:
[DEVELOPMENT.md](DEVELOPMENT.md)

## Releases changelog

Read the releases changelog in a separate md file:
[RELEASES.md](RELEASES.md)

## TODO

And code happily ever after...

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
