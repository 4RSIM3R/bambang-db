use std::io::{self, Write};

use figlet_rs::FIGfont;

mod file;

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
            _ if input.starts_with(".write") => {
                let path = input.split(" ").nth(1);
                if path.is_none() {
                    println!("Invalid path");
                    continue;
                }

                let path = path.unwrap();

                let result = file::write_file(path);
                if result.is_err() {
                    println!("Failed to write to {}", path);
                    continue;
                }

                println!("Wrote to {}", path);
            }
            _ if input.starts_with(".read") => {
                let path = input.split(" ").nth(1);
                if path.is_none() {
                    println!("Invalid path");
                    continue;
                }

                let path = path.unwrap();

                let result = file::read_file(path);
                if result.is_err() {
                    println!("Failed to read from {}", result.err().unwrap());
                    continue;
                }

                println!("Read from {}", path);
            }
            "" => {
                continue;
            }
            _ => {
                println!("Parse query: {}", input);
            }
        }
    }
}
