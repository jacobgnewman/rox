mod expr;
mod parser;
mod rox_type;
mod scanner;
mod token;
mod token_type;
mod interpreter;

use std::{env, result};
use std::fmt::Error;
use std::fs::read_to_string;
use std::io;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;


#[macro_use]
extern crate lazy_static;

fn main() {
    let mut interpreter = Interpreter::new();
    let mut args = env::args();
    if args.len() > 2 {
        println!("Usage: jlox [script]")
    } else if args.len() == 2 {
        let path =  &args.nth(1).expect("failed to get path argument");
        match run_file(path, &mut interpreter) {
            Ok(()) => (),
            Err(error) => panic!("Problem opening the file: {:?}", error),
        }
    } else {
        match run_prompt(&mut interpreter) {
            Ok(()) => (),
            Err(error) => panic!("Prompt failed: {:?}", error),
        }
    }
    if interpreter.had_runtime_error {
        std::process::exit(70);
    }
}

// Run source file
fn run_file(path: &str, interpreter: &mut Interpreter) -> Result<(), io::Error> {
    let code = read_to_string(path)?;

    match run(&code, interpreter) {
        Ok(()) => return Ok(()),
        Err(e) => panic!("{e}"),
    };
}

// Interactve shell
fn run_prompt(interpreter: &mut Interpreter) -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut buf = String::new();

    loop {
        print!("> ");
        stdin.read_line(&mut buf)?;
        if buf == "c\n" {
            break;
        } else {
            match run(&buf, interpreter) {
                Ok(_) => (),
                Err(e) => {
                    println!("{:?}", e);
                    return Ok(());
                }
            }
        }
        buf.drain(..);
    }
    return Ok(());
}

// Run Interpereter
fn run(code: &String, interpreter: &mut Interpreter) -> Result<(), Error> {

    let mut scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    if let Err(parse_err) = result {
        println!("Parse Error: {:?}", parse_err);
        return Ok(())
    }
    let result = result.unwrap();

    interpreter.interpret(result);
    
    return Ok(());
}

// Token enum for scanning
fn error(line: u32, message: &str) {
    report(line, "", message)
}

fn report(line: u32, location: &str, message: &str) {
    eprintln!("[line  {}] Error{}: {}", line, location, message)
}
