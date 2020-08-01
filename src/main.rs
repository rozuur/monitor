extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{Error};

fn pidfile_status(filename: &str) -> Result<File, Error> {
    // Rust doesn't have exceptions and uses Result type to pass
    // ? operator is a syntactic sugar for that makes error handling pleasant
    let file = File::open(filename)?;
    Ok(file)
}

// To use '?' which unwraps a result or propagates error, main function should return Result
fn main() -> Result<(), std::io::Error>{
    let matches = App::new("Monitor programs")
        .name("monitor")
        .version("0.1")
        .arg(
            Arg::with_name("status")
                .short("S")
                .long("status")
                .help("check status of --pidfile"),
        )
        .arg(
            Arg::with_name("pidfile")
                .short("p")
                .long("pidfile")
                .value_name("FILE")
                .help("write pid to <path>"),
        )
        .get_matches();

    if matches.is_present("status") {
        if !matches.is_present("pidfile") {
            eprintln!("Error: --pidfile required");
            std::process::exit(1);
            // Rust has panic and abort but they don't fail silently
        }
        let pidfile = matches.value_of("pidfile").unwrap_or("");
        let status = pidfile_status(pidfile)?;
        println!("{:?}", status);
    }

    Ok(())
}
