use clap::{Arg, Command};

mod problems;

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .arg(
            Arg::new("problem")
                .short('p')
                .long("problem")
                .required(true)
                .takes_value(true)
                .help("Number of the problem you want to run"),
        )
        .get_matches();

    problems::run_solver(matches.value_of("problem").unwrap().parse().unwrap());
}
