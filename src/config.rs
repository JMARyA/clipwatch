use clap::{App, Arg};

pub fn parse_config() -> Config {
    let args = App::new("Clipwatch")
        .version(option_env!("CARGO_PKG_VERSION").unwrap())
        .author("JMARyA <jmarya0@icloud.com>")
        .about("Keep an eye on the clipboard")
        .arg(
            Arg::with_name("ignore-first")
                .short("i")
                .long("ignore-first")
                .help("ignores the value stored in the clipboard at the start of the programm")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("summary")
                .short("s")
                .long("summary")
                .help("at termination of the programm a summary will be copied into the clipboard")
                .takes_value(false),
        )
        .get_matches();
    return Config::new(args);
}

#[derive(Debug)]
pub struct Config {
    pub ignore_first: bool,
    pub summary: bool,
}

impl Config {
    fn new(args: clap::ArgMatches) -> Config {
        return Config {
            ignore_first: args.is_present("ignore-first"),
            summary: args.is_present("summary"),
        };
    }
}
