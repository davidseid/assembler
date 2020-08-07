use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

fn main() {
    println!("Assembler starting up...");

    let args: Vec<String> = env::args().collect();

    println!("Parsing assembly file: {}", &args[1]);
    let file_parser = Parser::new(&args[1]);

    let _more = file_parser.has_more_commands();

    println!("{}",_more)
}

struct Parser {
    lines: Vec<String>,
}

impl Parser {
    fn new(filename: &str) -> Parser {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut lines = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {

                if line.starts_with("//") {
                    continue;
                }

                if line.is_empty() {
                    continue;
                }

                println!("{}", line);
                lines.push(line);
            }
        }

        Parser{
            lines,
        }
    }

    fn has_more_commands(&self) -> bool {
        if self.lines.len() > 0 {
            return true;
        }
        false
    }
}
