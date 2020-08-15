use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

fn main() {
    println!("Assembler starting up...");

    let args: Vec<String> = env::args().collect();

    println!("Parsing assembly file: {}", &args[1]);
    let mut file_parser = Parser::new(&args[1]);

    let _more = file_parser.has_more_commands();

    println!("{}",_more);

    file_parser.advance();

    file_parser.command_type();
}

struct Parser {
    lines: Vec<String>,
    current_command_index: Option<usize>
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
            current_command_index: None,
        }
    }

    fn has_more_commands(&self) -> bool {
        match self.current_command_index {
            Some(index) => index < self.lines.len(),
            None => true,
        }
    }

    fn advance(&mut self) {
        println!("{:?}", self.current_command_index);
        match self.current_command_index {
            Some(index) => self.current_command_index = Some(index+1),
            None => self.current_command_index = Some(0),
        }
    }

    fn command_type(&self) -> String {

        let index = self.current_command_index.unwrap();
        println!("{:?}", index);
        let current_command = &self.lines[index];
        println!("{:?}", current_command);

        if current_command.starts_with("@") {
            return String::from("A_COMMAND");
        }

        if current_command.contains("=") || current_command.contains(";") {
            return String::from("C_COMMAND");
        }

        String::from("L_COMMAND")
    }
}
