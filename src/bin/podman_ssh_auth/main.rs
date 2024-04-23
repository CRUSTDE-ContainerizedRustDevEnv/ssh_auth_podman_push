//! src/bin/podman_ssh_auth/main.rs

// Linux terminal colors
use podman_ssh_auth_lib::{GREEN, RESET, YELLOW};

/// entry point into the bin-executable
fn main() {
    std::panic::set_hook(Box::new(|panic_info| panic_set_hook(panic_info)));
    tracing_init();

    let Some(shell_command) = std::env::args().nth(1) else {
        panic!("The command arguments are not correct. It should look like: 'podman_ssh_auth login --username user_name registry' or 'podman_ssh_auth push registry/user_name/image_name:image_label'")
    };
    if shell_command == "login" {
        // podman_ssh_auth login --username user_name docker.io
        let Some(username_option) = std::env::args().nth(2) else {
            panic!("The command arguments are not correct. It should look like: 'podman_ssh_auth login --username user_name registry'")
        };
        if username_option != "--username" {
            panic!("The command arguments are not correct. It should look like: 'podman_ssh_auth login --username user_name registry'")
        }
        let Some(user_name) = std::env::args().nth(3) else {
            panic!("The command arguments are not correct. It should look like: 'podman_ssh_auth login --username user_name registry'")
        };
        let Some(registry) = std::env::args().nth(4) else {
            panic!("The command arguments are not correct. It should look like: 'podman_ssh_auth login --username user_name registry'")
        };
        podman_ssh_auth_lib::login(&user_name, &registry);
    } else if shell_command == "push" {
        // podman_ssh_auth push docker.io/bestiadev/crustde_cargo_img:cargo-xxx
        let Some(image_url) = std::env::args().nth(2) else {
            panic!("The command arguments are not correct. It should look like: 'podman_ssh_auth push registry/user_name/image_name:image_label'")
        };
        podman_ssh_auth_lib::push(&image_url);
    } else if shell_command == "--help" || shell_command == "-h" {
        print_help();
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
fn panic_set_hook(panic_info: &std::panic::PanicInfo) {
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
    {YELLOW}Use podman_ssh_auth to securely store and retrieve docker hub tokens !
{GREEN}podman_ssh_auth --help{RESET}
{GREEN}podman_ssh_auth login --username user_name docker.io{RESET}
{GREEN}podman_ssh_auth push docker.io/user_name/image_name:image_label{RESET}
  
    {YELLOW}© 2024 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
"#
    );
}
