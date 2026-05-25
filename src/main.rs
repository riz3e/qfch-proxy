use qfch::{print_usage, proxy_env_vars, PROXY_HOST, PROXY_PORT};
use std::env;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if config.cmd == "--help" || config.cmd == "-h" || config.cmd == "help" {
        print_usage();
        process::exit(0);
    }

    let mut child = Command::new(config.cmd);
    child.args(config.rest);

    for (key, val) in proxy_env_vars() {
        child.env(key, val);
    }

    eprintln!(
        "\x1b[2m[qfch] {} -> {}:{}\x1b[0m",
        config.cmd, PROXY_HOST, PROXY_PORT
    );

    child.stdin(process::Stdio::inherit());
    child.stdout(process::Stdio::inherit());
    child.stderr(process::Stdio::inherit());

    let status = match child.status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "\x1b[31m[qfch] failed to launch '{}': {}\x1b[0m",
                config.cmd, e
            );
            process::exit(127);
        }
    };

    process::exit(status.code().unwrap_or(1));
}

struct Config {
    cmd: String,
    rest: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let cmd = args[1].clone();
        let rest = args[2].clone();

        Ok(Config { cmd, rest })
    }
}
