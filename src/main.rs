use std::io::{self, Write};
use notes2::parse_tag;

fn main() {
    println!("Enter interactive tags (empty line to quit):");
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        print!("> ");
        let _ = io::stdout().flush();
        if stdin.read_line(&mut line).is_err() {
            eprintln!("failed to read line");
            break;
        }
        let input = line.trim();
        if input.is_empty() {
            break;
        }
        match parse_tag(input) {
            Some(tag) => println!("parsed: {:?}", tag),
            None => println!("invalid tag"),
        }
    }
}
