#![feature(option_result_contains)]

pub mod error;
pub mod scanner;
pub mod token_type;
pub mod token;

use error::{LoxError, LoxResult};
use scanner::*;

use std::env;
use std::io;
use std::io::Write;

use std::fs;
use std::path::Path;
use std::str;

fn print_usage() {
    println!("Usage: rlox [script]");
}

fn run(source: String) -> LoxResult {
    let mut scanner = Scanner::new();
    let tokens = scanner.scan_tokens(source);

    dbg!(tokens);
    Ok(())
}

fn run_prompt() -> LoxResult {
    loop {
        let mut line = String::new();

        print!("rlox > ");
        io::stdout().flush().expect("Failed to flush stdout");

        match io::stdin().read_line(&mut line) {
            // this needs to exit the program and it currently doesn'))
            Ok(0) => break Ok(()),
            Ok(_) => run(line)?,
            Err(_) => panic!("Failed to read line!"),
        }
    }
}

fn run_file(path: &str) -> LoxResult {
    let path = Path::new(path);
    let display = path.display();

    let source = match &fs::read(&path) {
        Ok(bytes) => bytes.clone(),
        Err(e) => panic!("couldn't open {}: {}", display, e),
    };

    let source = match str::from_utf8(&source) {
        Ok(v) => String::from(v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    run(source)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let result = match &args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => Err(LoxError {
            line: -1,
            place: String::from(""),
            message: String::from("Wrong number of args"),
        }),
    };

    let status = match result {
        Ok(()) => 0,
        Err(e) => {
            if e.line == -1 {
                print_usage();
                64
            } else {
                65
            }
        }
    };

    std::process::exit(status);
}
