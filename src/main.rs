extern crate clap;

use clap::{App, Arg};
use std::convert::TryInto;
use std::fs::File;
use std::io::{stdin, Error, Read};

fn pidfile_status(filename: &str) -> Result<u32, Error> {
    // Defining buffer as let buf_size = 64 fails, array initialization requires it as constant
    // And BUF_SIZE type should be usize, if not specified number will be treated as integer i32
    const BUF_SIZE: usize = 64;
    // Following creates an integer array of size 256 and filled with 0's
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    // Rust doesn't have exceptions and uses Result type to pass
    // ? operator is a syntactic sugar for that makes error handling pleasant
    let mut file = File::open(filename)?;
    let size = file.read(&mut buf[0..BUF_SIZE])?;
    /*
    Trying to parse an pid from file

    There are functions to read a complete line and parse it to read an integer, read_line
    But reading complete line is not required as to parse an integer

    from_be_bytes takes [u8; 4] and there is no trivial way to convert an array to fixed size array
     */

    let pid = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
    Ok(pid)
}

// To use '?' which unwraps a result or propagates error, main function should return Result
fn main() -> Result<(), std::io::Error> {
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

    /*
     If not status and pidfile is present, write current process pid into it
     Use magic number when writing, to not read pid from arbitrary file

     Use same pidfile and display stats
     */

    Ok(())
}
