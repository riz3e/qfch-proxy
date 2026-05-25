use qfch::{print_usage, run};
use std::env;
use std::process;

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

    if let Err(e) = run(config.cmd, config.rest, PROXY_HOST, PROXY_PORT) {
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
