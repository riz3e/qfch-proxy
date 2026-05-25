pub const PROXY_HOST: &str = "127.0.0.1";
pub const PROXY_PORT: u16 = 10808;

pub fn proxy_env_vars() -> Vec<(&'static str, String)> {
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

pub fn print_usage() {
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
