extern crate clap;
extern crate nix;

use clap::{App, Arg, ArgMatches};
use nix::sys::signal::kill;
use nix::unistd::Pid;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::time::Duration;

#[derive(Debug)]
struct PidStatus {
    pid: u32,
    is_alive: bool,
    uptime: Duration,
}

fn write_pid(pid: u32, filename: &str) -> std::io::Result<()> {
    // Array's doesn't implement + so concatenation works as shown,
    // also slicing is used to convert from fixed array
    std::fs::write(filename, pid.to_string())
}

// std::io::Result is type alias for std::result::Result<T, io::Error>
fn pidfile_status(filename: &str) -> std::io::Result<PidStatus> {
    // Defining buffer as let buf_size = 64 fails, array initialization requires it as constant
    // And BUF_SIZE type should be usize, if not specified number will be treated as integer i32
    const BUF_SIZE: usize = 16;
    // Following creates an integer array of size 256 and filled with 0's
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    // Rust doesn't have exceptions and uses Result type to pass
    // ? operator is a syntactic sugar for that makes error handling pleasant
    let mut file = File::open(filename)?;
    let size = file.read(&mut buf[0..BUF_SIZE])?;

    // FIXME std::io::Result
    println!("{:?}", std::str::from_utf8(&buf));
    let pid: u32 = std::str::from_utf8(&buf[..size])
        .map_err(|_e| std::io::ErrorKind::Other)?
        .parse()
        .map_err(|_e| std::io::ErrorKind::Other)?;

    // Kill command is not present in standard library, so need to use nix crate.
    // What is try_into and unwrap from u32 to i32?
    let kill_status = kill(Pid::from_raw(pid.try_into().unwrap()), None);
    // Returning  elapsed time is not working as Result is not a generic error.
    let stat = std::fs::metadata(filename)?;
    // TODO fix error
    let uptime = stat
        .created()?
        .elapsed()
        .map_err(|_e| std::io::ErrorKind::Other)?;
    Ok(PidStatus {
        pid,
        is_alive: kill_status.is_ok(),
        uptime,
    })
}

fn print_status(matches: &ArgMatches) -> Result<(), std::io::Error> {
    // TODO take pidfile only
    let pidfile = matches.value_of("pidfile");
    if pidfile.is_none() {
        eprintln!("Error: --pidfile required");
        std::process::exit(1);
        // Rust has panic and abort but they don't fail silently
    }
    // TODO is this an idiomatic way of unwrapping after checking?
    let pidfile = pidfile.unwrap();
    let status = pidfile_status(pidfile)?;
    // 61406 : alive : uptime 27 seconds
    if status.is_alive {
        println!("{} : alive : uptime {:?}", status.pid, status.uptime);
    } else {
        println!("{} : dead", status.pid);
    }

    Ok(())
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
        .arg(
            Arg::with_name("mon-pidfile")
                .short("m")
                .long("mon-pidfile")
                .value_name("FILE")
                .help("write mon pid to <path>"),
        )
        .arg(
            Arg::with_name("command")
                .value_name("CMD")
                .help("command to be executed"),
        )
        .get_matches();

    if matches.is_present("status") {
        print_status(&matches)?;
        return Ok(());
    }

    if matches.is_present("mon-pidfile") {
        // Get pid of current process
        let pid = std::process::id();
        write_pid(pid, matches.value_of("mon-pidfile").unwrap())?;
        return Ok(());
    }

    // Doesn't clap provide error messages if required?
    let command = matches.value_of("command").expect("command is required");
    Command::new("sh").arg("-c").arg(command).spawn()?;

    Ok(())
}
