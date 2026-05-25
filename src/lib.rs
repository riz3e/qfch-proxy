fn proxy_env_vars(proxy_host: &str, proxy_port: u16) -> Vec<(&'static str, String)> {
    let socks5 = format!("socks5h://{}:{}", proxy_host, proxy_port);
    let http = format!("socks5h://{}:{}", proxy_host, proxy_port);

    vec![
        ("ALL_PROXY", socks5.clone()),
        ("all_proxy", socks5.clone()),
        ("HTTP_PROXY", http.clone()),
        ("http_proxy", http.clone()),
        ("HTTPS_PROXY", http.clone()),
        ("https_proxy", http.clone()),
        ("GIT_HTTP_PROXY_AUTHMETHOD", "basic".to_string()),
    ]
}

pub fn print_usage(proxy_host: &str, proxy_port: u16) {
    eprintln!(
        "qfch — run a command through v2rayN SOCKS5 proxy ({}:{})",
        proxy_host, proxy_port
    );
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  qfch <command> [args...]");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  qfch git push -u origin main");
    eprintln!("  qfch curl https://example.com");
    eprintln!("  qfch cargo add serde");
    eprintln!("  qfch wget https://example.com/file.zip");
}

use std::error::Error;

pub fn run(
    cmd_str: String,
    rest_args: Vec<String>,
    proxy_host: &str,
    proxy_port: u16,
) -> Result<(), Box<dyn Error>> {
    use std::process::{self, Command};

    if matches!(cmd_str.as_str(), "--help" | "-h" | "help") {
        print_usage(proxy_host, proxy_port);
        process::exit(0);
    }

    let mut cmd = Command::new(&cmd_str);

    eprintln!(
        "\x1b[2m[qfch] {} -> {}:{}\x1b[0m",
        cmd_str, proxy_host, proxy_port
    );

    let status = cmd
        .args(rest_args)
        .envs(proxy_env_vars(proxy_host, proxy_port))
        .stdin(process::Stdio::inherit())
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .status()?;

    process::exit(status.code().unwrap_or(1));
}
