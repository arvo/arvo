//! # Arvo

extern crate arvo;
extern crate getopts;

pub use arvo;

use getopts::Options;
use std::env;
use std::process::{Command, Stdio};
use std::io::{ErrorKind};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("v", "version", "print the version of arvo");
    opts.optflag("h", "help", "print this help menu");
    
    run(&args, &program, &opts);
}

fn run(args: &Vec<String>, program: &String, opts: &Options) {
    let matches = match opts.parse(&args[1..]) {
        Ok(ok) => { ok }
        Err(..) => { 
            print_usage(program, opts);
            return;
        }
    };
    if matches.opt_present("h") {
        print_usage(program, opts);
        return;
    }
    if matches.opt_present("v") {
        print_version();
        return;
    }
    if !matches.free.is_empty() {
        let mut command_dir = match env::current_exe() {
            Ok(path) => path,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        command_dir.pop();
        let result = Command::new(format!("{}/{}", command_dir.to_str().unwrap(), &matches.free[0]))
            .args(&args[2..])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();
        match result {
            Ok(..) => (),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    println!("Subcommand '{}' not found.\nSee'{} --help' for usage.", &matches.free[0], program);
                } else {
                    println!("{}", err);
                }
            },
        };
    } else {
        print_usage(program, opts);
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} SUBCOMMAND [options]", program);
    println!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("Arvo version 0.1.0");
}