fn proxy_env_vars(proxy_host: &str, proxy_port: u16) -> Vec<(&'static str, String)> {
    let proxy = format!("socks5h://{}:{}", proxy_host, proxy_port);

    vec![
        ("ALL_PROXY", proxy.clone()),
        ("all_proxy", proxy.clone()),
        ("HTTP_PROXY", proxy.clone()),
        ("http_proxy", proxy.clone()),
        ("HTTPS_PROXY", proxy.clone()),
        ("https_proxy", proxy.clone()),
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
    eprintln!("  qfch [--port <port>] <command> [args...]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --port <port>    Proxy port (default: 10808 for v2rayN, use 7890 for Clash)");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  qfch git push -u origin main");
    eprintln!("  qfch --port 7890 curl https://example.com");
    eprintln!("  qfch cargo add serde");
    eprintln!("  qfch --port 7890 wget https://example.com/file.zip");
}

use std::task::Wake;
use std::{error::Error, fmt::format};

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

use std::net::TcpStream;
use std::time::Duration;

// PORT CHECKING

pub fn is_port_open(host: &str, port: u16) -> bool {
    let addr = format!("{}:{}", host, port);
    TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(200)).is_ok()
}

const KNOWN_PROXY_PORTS: &[u16] = &[10808, 7890, 1080, 10809];

pub fn detect_proxy_port(host: &str) -> Option<u16> {
    for port in KNOWN_PROXY_PORTS {
        if is_port_open(host, *port) {
            return Some(*port);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proxy_env_vars_count() {
        let proxies = proxy_env_vars("localhost", 1080);
        assert_eq!(proxies.len(), 7);
    }

    #[test]
    fn proxy_env_vars_socks5h_scheme() {
        let proxies = proxy_env_vars("127.0.0.1", 1080);
        for (key, val) in &proxies {
            if *key != "GIT_HTTP_PROXY_AUTHMETHOD" {
                assert!(
                    val.starts_with("socks5h://"),
                    "expected socks5h:// in {key}={val}"
                );
            }
        }
    }

    #[test]
    fn proxy_env_vars_git_authmethod() {
        let proxies = proxy_env_vars("localhost", 1080);
        let git_auth = proxies
            .iter()
            .find(|(k, _)| *k == "GIT_HTTP_PROXY_AUTHMETHOD")
            .map(|(_, v)| v.as_str());
        assert_eq!(git_auth, Some("basic"));
    }
}
