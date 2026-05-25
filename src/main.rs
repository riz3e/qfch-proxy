use qfch::proxy_env_vars;
use std::env;
use std::error::Error;
use std::process::{self, Command};

const PROXY_HOST: &str = "127.0.0.1";
const PROXY_PORT: u16 = 10808;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(PROXY_HOST, PROXY_PORT);
        process::exit(1);
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("\x1b[31m[qfch] error: {}\x1b[0m", e);
        process::exit(127);
    }
}

struct Config {
    cmd: String,
    rest: Vec<String>,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let cmd = args[1].clone();
        let rest = args[2..].to_vec();

        Ok(Config { cmd, rest })
    }
}

fn print_usage(proxy_host: &str, proxy_port: u16) {
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.cmd == "--help" || config.cmd == "-h" || config.cmd == "help" {
        print_usage(PROXY_HOST, PROXY_PORT);
        process::exit(0);
    }

    let mut cmd = Command::new(&config.cmd);
    cmd.args(config.rest);

    for (key, val) in proxy_env_vars(PROXY_HOST, PROXY_PORT) {
        cmd.env(key, val);
    }

    eprintln!(
        "\x1b[2m[qfch] {} -> {}:{}\x1b[0m",
        config.cmd, PROXY_HOST, PROXY_PORT
    );

    cmd.stdin(process::Stdio::inherit())
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit());

    let status = cmd.status()?;

    process::exit(status.code().unwrap_or(1));
}
