extern crate clap;

fn main() {
    clap::App::new("Monitor programs").name("monitor").version("0.1").get_matches();
}
