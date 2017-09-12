//! # Arvo

extern crate arvoc;
extern crate getopts;

use arvoc::compile;
use arvoc::repl;
use getopts::Options;
use std::env;

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    let config = parse_arguments(&arguments);
    run(&config);
}

struct Config<'a> {
    program_name: &'a String,
    program_opts: Options,
    program_subcommand: Option<String>,
}

fn parse_arguments<'a>(program_args: &'a Vec<String>) -> Config<'a> {

    let name = &program_args[0];
    let mut opts = Options::new();
    opts.optflag("v", "version", "print the version of arvo");
    opts.optflag("h", "help", "print this help menu");

    let mut config = Config {
        program_name: name,
        program_opts: opts,
        program_subcommand: None,
    };

    let matches = match config.program_opts.parse(&program_args[1..]) {
        Ok(ok) => ok,
        Err(..) => {
            print_usage(&config.program_name, &config.program_opts);
            return config;
        }
    };

    config.program_subcommand = if matches.opt_present("h") {
        Some("help".to_string())
    } else if matches.opt_present("v") {
        Some("version".to_string())
    } else if !matches.free.is_empty() {
        Some(matches.free[0].clone())
    } else {
        None
    };

    return config;
}

fn run(config: &Config) {
    if let Some(ref subcommand) = config.program_subcommand {
        if subcommand == "help" {
            print_usage(&config.program_name, &config.program_opts);
        } else if subcommand == "version" {
            print_version();
        } else if subcommand == "compile" {
            compile();
        } else {
            print_usage(&config.program_name, &config.program_opts);
        }
    } else {
        print_version();
        repl();
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} SUBCOMMAND [options]", program);
    println!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("Arvo version 0.1.0");
}