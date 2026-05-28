use qfch::{print_usage, run};
use std::env;
use std::process;

const PROXY_HOST: &str = "127.0.0.1";
const DEFAULT_PROXY_PORT: u16 = 10808;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(PROXY_HOST, DEFAULT_PROXY_PORT);
        process::exit(1);
    }

    let config = Config::new(&args);

    if let Err(e) = run(config.cmd, config.rest, PROXY_HOST, config.port) {
        eprintln!("\x1b[31m[qfch] error: {}\x1b[0m", e);
        process::exit(127);
    }
}

struct Config {
    cmd: String,
    rest: Vec<String>,
    port: u16,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let mut port = DEFAULT_PROXY_PORT;
        let mut cmd_idx = 1;

        // Check for --port flag
        if args.len() > 2 && args[1] == "--port" {
            if args.len() < 4 {
                eprintln!("\x1b[31m[qfch] error: --port requires a value\x1b[0m");
                process::exit(1);
            }
            port = args[2].parse().unwrap_or_else(|_| {
                eprintln!(
                    "\x1b[31m[qfch] error: invalid port number '{}'\x1b[0m",
                    args[2]
                );
                process::exit(1);
            });
            cmd_idx = 3;
        }

        Config {
            cmd: args[cmd_idx].clone(),
            rest: args[cmd_idx + 1..].to_vec(),
            port,
        }
    }
}
