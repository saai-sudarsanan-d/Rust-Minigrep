use std::{env, process};
use minigrep::{Config,documentation};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        documentation();
        eprintln!("Problem passing arguements : {}",err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}",e);
        process::exit(1);
    }
}