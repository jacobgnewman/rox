use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};



fn main() -> io::Result<()> {
    let mut args = env::args();

    if args.len() > 2 {
        println!("Usage: jlox [script]")
    } else if args.len() == 2 {
        run_file(&args.nth(1).unwrap())?;
    } else {
        run_prompt()?;
    }

    return Ok(());
}

// Run source file
fn run_file(path: &str) -> io::Result<()> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    for line in f.lines() {
        run(&line?);
    }

    return Ok(());
}

// Interactve shell
fn run_prompt() -> Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut handle = stdin.lock();

    loop {
        print!("> ");
        stdin.read_line(&mut buf)?;
        if buf == "c\n" {
            break;
        } else {
            run(&buf)
        }
        buf.drain(..);
    }
    return Ok(());
}

struct Scanner {}

impl Scanner {
    pub fn scan_tokens(code: &String) -> Vec<Token> {
        let tokens = Vec::<Token>::new();

        for segment in code.chars() {
            let mut token = String::new();
        }
        return tokens
    }
}

// Run Interpereter
fn run(code: &String) {
    let tokens = Scanner::scan_tokens(&code);
    for token in tokens {
        println!("{}", token);
    }
    print!("{}", code)
}

struct Token {
    
}
// print out for tokens
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "twest")
    }
}

fn error(line: u8, message: &str){
    report(line, "", message)
}
fn report(line: u8, location: &str, message: &str) {
    eprintln!("[line  {}] Error{}: {}", line, location, message)
}