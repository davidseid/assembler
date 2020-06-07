use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::env;

fn main() {
    println!("Assembler starting up...");

    let args: Vec<String> = env::args().collect();

    println!("Parsing assembly file: {}", &args[1]);
    let fileParser = Parser::new("./Add.asm");

    let more = fileParser.has_more_commands();
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

    fn has_more_commands(&self) -> bool {
        println!("{:?}", self.lines);
        true
    }
}
