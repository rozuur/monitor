extern crate clap;

use clap::{App, Arg};

fn show_status(filename: &str) {
    println!("Show status of {}", filename)
}

fn main() {
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
        }
        let pidfile = matches.value_of("pidfile").unwrap_or("");
        show_status(pidfile);
        std::process::exit(0);
    }
}
