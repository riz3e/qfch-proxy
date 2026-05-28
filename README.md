# qfch

> **Personal tool.** Built for living in China behind the GFW — wraps any command through a local [v2rayN](https://github.com/2dust/v2rayN) SOCKS5 proxy so you don't have to think about it.

---

## What it does

`qfch` runs any command with proxy environment variables pre-set, pointing at your local v2rayN SOCKS5 listener. No config files, no wrappers, no manual `export`. Just prefix your command and go.

```sh
qfch cargo add tokio
qfch git push origin main
qfch curl https://api.github.com
```

---

## Why it exists

Living with the Great Firewall means tools like `cargo`, `git`, `wget`, and `curl` silently fail or time out when they try to reach foreign servers. v2rayN solves this at the network level, but most CLI tools don't respect the system proxy automatically — they need explicit environment variables.

`qfch` injects all the relevant proxy env vars (`ALL_PROXY`, `HTTP_PROXY`, `HTTPS_PROXY`, and their lowercase variants) set to `socks5h://127.0.0.1:10808` — the default v2rayN SOCKS5 port — and then hands control over to your command.

---

## Requirements

- [v2rayN](https://github.com/2dust/v2rayN) running locally with SOCKS5 on port `10808`
- Rust toolchain (to build `qfch`)

---

## Installation (didn't tested yet)

```sh
cargo install --path .
```

Or build manually:

```sh
cargo build --release
# binary will be at ./target/release/qfch
# copy it somewhere on your PATH, e.g.:
cp target/release/qfch ~/.local/bin/
```

---

## Usage

```
qfch [--port <port>] <command> [args...]
```

### Options

- `--port <port>` - Specify proxy port (default: 10808 for v2rayN, use 7890 for Clash)

### Examples

```sh
# Rust / Cargo (default v2rayN port)
qfch cargo add serde
qfch cargo update

# Git with Clash proxy
qfch --port 7890 git push -u origin main
qfch --port 7890 git clone https://github.com/user/repo

# curl / wget
qfch curl https://example.com
qfch --port 7890 wget https://example.com/file.zip

# Anything else
qfch npm install
qfch pip install requests
```

### Help

```sh
qfch --help
```

---

## Configuration

Default proxy settings:
- Host: `127.0.0.1` (hardcoded in `src/main.rs`)
- Port: `10808` (v2rayN default, can be overridden with `--port`)

Common ports:
- `10808` - v2rayN default SOCKS5 port
- `7890` - Clash default SOCKS5 port

Use `--port` flag to switch between different proxy configurations without rebuilding.

---

## How it works

`qfch` sets the following environment variables before spawning your command:

| Variable | Value |
|---|---|
| `ALL_PROXY` / `all_proxy` | `socks5h://127.0.0.1:10808` |
| `HTTP_PROXY` / `http_proxy` | `socks5h://127.0.0.1:10808` |
| `HTTPS_PROXY` / `https_proxy` | `socks5h://127.0.0.1:10808` |
| `GIT_HTTP_PROXY_AUTHMETHOD` | `basic` |

`socks5h` means DNS resolution also goes through the proxy — important for avoiding DNS leaks and SNI-based blocking.

---

## License

Personal use. Do whatever you want with it.
