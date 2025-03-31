//! src/bin/ssh_auth_podman_push/main.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # ssh_auth_podman_push
//!
//! **Store and use encrypted docker-hub secret_token with SSH key**  
//! ***version: 1.1.1 date: 2024-04-30 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/CRUSTDE-ContainerizedRustDevEnv/ssh_auth_podman_push)***
//!
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!  ![docker-hub](https://img.shields.io/badge/docker_hub-orange)
//!
//!   [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/ssh_auth_podman_push/blob/main/LICENSE)
//!   [![Rust](https://github.com/CRUSTDE-ContainerizedRustDevEnv/ssh_auth_podman_push/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/ssh_auth_podman_push/)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-265-green.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/ssh_auth_podman_push/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-270-blue.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/ssh_auth_podman_push/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-59-purple.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/ssh_auth_podman_push/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/ssh_auth_podman_push/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-193-orange.svg)](https://github.com/CRUSTDE-ContainerizedRustDevEnv/ssh_auth_podman_push/)
//!
//! Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//! I recommend using the [CRUSTDE - Containerized Rust Development Environment](https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod) to write Rust projects on Linux, isolated from your system.  
//!
//! ## Motivation
//!
//! To access docker-hub you need a username+password or an access secret_token.  
//! IMPORTANT: Treat access secret_tokens like your password and keep them secret. Store your secret_tokens securely in a credential manager for example.  
//! Access secret_tokens are impossible to remember for an average human. We need to store them somewhere.  
//! FYI: Podman is an alternative "drop-in replacement" for Docker.  
//! I am sure they both store the docker-hub secret_token for login with the command:
//!
//! ```bash
//! podman login --username user_name docker.io
//! docker login --username user_name docker.io
//! ```
//!
//! WARNING: Be aware that they store the secret_token in "plain-text" in the file: `${XDG_RUNTIME_DIR}/containers/auth.json`.  
//! Ok, it is not really plain-text, but base64 encoding is not a security feature.  
//! This means that every attacker that can get to this well-known file, can log in to our Docker Hub account. No bueno!!!
//!
//! I want to secure this secret_token with encryption with an SSH key.  
//! We have already a lot of experience creating, managing and securing our SSH keys. The private key is secured by a passphrase we can remember and type. Every use of the secret_token will need user interaction to type the passphrase. Very secure.  
//!
//! If we are very self-confident in our current session, we can store the SSH key in ssh-agent and write our passphrase only once.  
//! WARNING: a dedicated attacker could read from ssh-agent and discover the access secret_token without our user interaction. Use this at your discretion.  
//!
//! ## Replacement command
//!
//! Put the executable `ssh_auth_podman_push` into the folder you intend to use it.  
//! After copying, make it executable with `chmod +x ssh_auth_podman_push`.  
//! Instead of `podman push...` use `ssh_auth_podman_push`.  
//! If it finds the encrypted secret_token it will ask you for the passphrase to the private SSH key.
//! Else it will ask you to store the secret_token.
//!
//! ## Development details
//!
//! Read the development details in a separate md file:
//! [DEVELOPMENT.md](DEVELOPMENT.md)
//!
//! ## Releases changelog
//!
//! Read the releases changelog in a separate md file:
//! [RELEASES.md](RELEASES.md)
//!
//! ## TODO
//!
//! And code happily ever after...
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// Linux terminal colors
use ssh_auth_podman_push_lib::{GREEN, RESET, YELLOW};

/// entry point into the bin-executable
fn main() {
    std::panic::set_hook(Box::new(|panic_info| panic_set_hook(panic_info)));
    tracing_init();

    let Some(arg_first) = std::env::args().nth(1) else {
        panic!("The command arguments are not correct. It should look like: 'ssh_auth_podman_push registry/user_name/image_name:image_label'")
    };
    if arg_first == "--help" || arg_first == "-h" {
        // ssh_auth_podman_push --help
        print_help();
    } else {
        // ssh_auth_podman_push docker.io/bestiadev/crustde_cargo_img:cargo-xxx
        ssh_auth_podman_push_lib::push(&arg_first);
    }
}

// region: general functions

/// Initialize tracing to file logs/automation_tasks_rs.log
///
/// The folder logs/ is in .gitignore and will not be committed.
pub fn tracing_init() {
    // uncomment this line to enable tracing to file
    // let file_appender = tracing_appender::rolling::daily("logs", "automation_tasks_rs.log");

    let offset = time::UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(offset, time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:6]"));

    // Filter out logs from: hyper_util, reqwest
    // A filter consists of one or more comma-separated directives
    // target[span{field=value}]=level
    // examples: tokio::net=info
    // directives can be added with the RUST_LOG environment variable:
    // export RUST_LOG=automation_tasks_rs=trace
    // Unset the environment variable RUST_LOG
    // unset RUST_LOG
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("hyper_util=error".parse().unwrap_or_else(|e| panic!("{e}")))
        .add_directive("reqwest=error".parse().unwrap_or_else(|e| panic!("{e}")));

    tracing_subscriber::fmt()
        .with_file(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_timer(timer)
        .with_line_number(true)
        .with_ansi(false)
        //.with_writer(file_appender)
        .with_env_filter(filter)
        .init();
}

/// The original Rust report of the panic is ugly for the end user
///
/// I use panics extensively to stop the execution. I am lazy to implement a super complicated error handling.
/// I just need to stop the execution on every little bit of error. This utility is for developers. They will understand me.
/// For errors I print the location. If the message contains "Exiting..." than it is a "not-error exit" and  the location is not important.
fn panic_set_hook(panic_info: &std::panic::PanicHookInfo) {
    let mut string_message = "".to_string();
    if let Some(message) = panic_info.payload().downcast_ref::<String>() {
        string_message = message.to_owned();
    }
    if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
        string_message.push_str(message);
    }

    tracing::debug!("{string_message}");
    eprintln!("{string_message}");

    if !string_message.contains("Exiting...") {
        let file = panic_info.location().unwrap().file();
        let line = panic_info.location().unwrap().line();
        let column = panic_info.location().unwrap().column();
        tracing::debug!("Location: {file}:{line}:{column}");
        eprintln!("Location: {file}:{line}:{column}");
    }
}

// endregion: general functions

/// print help
fn print_help() {
    println!(
        r#"
    {YELLOW}Use ssh_auth_podman_push to securely store and retrieve docker hub secret_tokens !
{GREEN}ssh_auth_podman_push --help{RESET}
{GREEN}ssh_auth_podman_push docker.io/user_name/image_name:image_label{RESET}
  
    {YELLOW}© 2025 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
"#
    );
}
