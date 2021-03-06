use std::{env, process};

use minigrep::{run, Config};

fn main() {
    // env::args 读取 unicode 可表示参数
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("{err}");
        process::exit(1);
    }
}
