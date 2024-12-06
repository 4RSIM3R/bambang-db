use std::io::{self, Write};

use figlet_rs::FIGfont;
use file::Bambang;

mod file;
mod parser;

fn main() {
    let font = FIGfont::standard().unwrap();
    let display = font.convert("BAMBANG DB");

    println!("{}", display.unwrap());

    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            ".quit" => {
                println!("Quitting...");
                break;
            }
            _ if input.starts_with(".write") => {}
            _ if input.starts_with(".read") => {
                let path = input.split(" ").nth(1);
                if path.is_none() {
                    println!("Invalid path");
                    continue;
                }

                let path = path.unwrap();
                let result = Bambang::new(path);

                if result.is_err() {
                    println!("Failed to read from {}", result.err().unwrap());
                    continue;
                }

                result.unwrap().print_header();
            }
            "" => {
                continue;
            }
            query => {
                let result = parser::select::parse_select(query);
                if result.is_err() {
                    println!("{}", result.err().unwrap());
                    continue;
                }

                let query = result.unwrap().1;

                println!("{:#?}", query);
            }
        }
    }
}
