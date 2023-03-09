use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Applicatoin error: {e}");
        process::exit(1);
    }
}


// PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
// PS> Remove-Item Env:IGNORE_CASE