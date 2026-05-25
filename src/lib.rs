pub fn proxy_env_vars(proxy_host: &str, proxy_port: u16) -> Vec<(&'static str, String)> {
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
