use std::io::{self, BufReader, BufRead};
use std::fs::File;

fn main() {
    println!("Assembler starting up");
    let myParser = Parser::new("./Add.asm");
}

struct Parser {
    lines: io::Lines<BufReader<File>>,
}

impl Parser {
    fn new(filename: &str) -> Parser {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        Parser{
            lines: reader.lines(),
        }
    }
}
