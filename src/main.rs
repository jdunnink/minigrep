use std::process;           // gives access to process module for function exit
use std::env;               // gives access to env module for command line arguments
use minigrep::Config;        // gives access to Config struct

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);                                        // eprintln! directs to stderr instead of stdout 
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {                                                   // in case of OK(()) return value, use 'if let' syntax instead of .unwrap_or_else()
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
