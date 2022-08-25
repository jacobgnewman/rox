mod rox_type;
mod scanner;
mod token;
mod token_type;
mod expr;

use std::env;
use std::fmt::Error;
use std::fs::read_to_string;
use std::io;

//use token::Token;
//use token_type::TokenType;
use scanner::Scanner;


#[macro_use]
extern crate lazy_static;



fn main() {
    let mut args = env::args();
    if args.len() > 2 {
        println!("Usage: jlox [script]")
    } else if args.len() == 2 {
        match run_file(&args.nth(1).unwrap()) {
            Ok(()) => (),
            Err(error) => panic!("Problem opening the file: {:?}", error)
        }
    } else {
        match run_prompt() {
            Ok(()) => (),
            Err(error) => panic!("Prompt failed: {:?}", error)
        }

    }
}

// Run source file
fn run_file(path: &str) -> Result<(), io::Error> {
    let code = read_to_string(path)?;

    match run(&code) {
        Ok(()) => return Ok(()),
        Err(e) => panic!("{e}")
    };
}

// Interactve shell
fn run_prompt() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut buf = String::new();

    loop {
        print!("> ");
        stdin.read_line( &mut buf)?;
        if buf == "c\n" {
            break;
        } else {
            run(&buf); //handle error?
        }
        buf.drain(..);
    }
    return Ok(());
}

// Run Interpereter
fn run(code: &String) -> Result<(), Error> {
    let mut scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
    return Ok(())
}


// Token enum for scanning

fn error(line: u32, message: &str){
    report(line, "", message)
}
fn report(line: u32, location: &str, message: &str) {
    eprintln!("[line  {}] Error{}: {}", line, location, message)
}