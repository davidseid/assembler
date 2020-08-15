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

enum Command {
    ACommand,
    CCommand,
    LCommand,
}

struct Parser {
    lines: Vec<String>,
    current_command_index: Option<usize>,
    current_command: Option<String>,
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
            current_command: None,
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
            Some(index) => {
                self.current_command_index = Some(index+1);
            },
            None => {
                self.current_command_index = Some(0);
            }
        }

        let command_ref = &self.lines[self.current_command_index.unwrap()];
        self.current_command = Some(String::from(command_ref));

    }

    fn command_type(&self) -> Command {

        let command = self.current_command.as_ref().unwrap();
        if command.starts_with("@") {
            return Command::ACommand;
        }

        if command.contains("=") || command.contains(";") {
            return Command::CCommand;
        }

        Command::LCommand
    }

    fn symbol(&self) -> String {
        let command = self.current_command.as_ref().unwrap();
        match self.command_type() {
            Command::ACommand => command.trim_start_matches("@").to_string(),
            Command::LCommand => command.to_string(),
            _ => command.to_string(),
        }
    }

    fn dest(&self) -> String {
        let command = self.current_command.as_ref().unwrap();
        command.to_string().split("=").collect::<Vec<&str>>().first().unwrap().to_string()
    }
}
