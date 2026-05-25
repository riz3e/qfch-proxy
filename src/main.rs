use std::env;
use std::process::{self, Command};

const PROXY_HOST: &str = "127.0.0.1";
const PROXY_PORT: u16 = 10808;

fn proxy_env_vars() -> Vec<(&'static str, String)> {
    let socks5 = format!("socks5h://{}:{}", PROXY_HOST, PROXY_PORT);
    let http = format!("socks5h://{}:{}", PROXY_HOST, PROXY_PORT);

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

fn print_usage() {
    eprintln!(
        "qfch — run a command through v2rayN SOCKS5 proxy ({}:{})",
        PROXY_HOST, PROXY_PORT
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let cmd = &args[1];
    let rest = &args[2..];

    if cmd == "--help" || cmd == "-h" || cmd == "help" {
        print_usage();
        process::exit(0);
    }

    let mut child = Command::new(cmd);
    child.args(rest);

    for (key, val) in proxy_env_vars() {
        child.env(key, val);
    }

    eprintln!(
        "\x1b[2m[qfch] {} -> {}:{}\x1b[0m",
        cmd, PROXY_HOST, PROXY_PORT
    );

    child.stdin(process::Stdio::inherit());
    child.stdout(process::Stdio::inherit());
    child.stderr(process::Stdio::inherit());

    let status = match child.status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\x1b[31m[qfch] failed to launch '{}': {}\x1b[0m", cmd, e);
            process::exit(127);
        }
    };

    process::exit(status.code().unwrap_or(1));
}
