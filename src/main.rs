use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

fn main() {
    println!("Assembler starting up...");

    let args: Vec<String> = env::args().collect();

    println!("Parsing assembly file: {}", &args[1]);
    let mut file_parser = Parser::new(&args[1]);

    // while it has more lines
    // print each line
    // print get the command type
    // if the command type is A or L
    // get the symbol
    // if the command type is C
    // get comp, dest, jump
    // advance

    while file_parser.has_more_commands() {
        file_parser.advance();
    }

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

        println!("{:?}", self.current_command_index);
        match self.current_command_index {
            Some(index) => index < self.lines.len(),
            None => true,
        }
    }

    fn advance(&mut self) {
        match self.current_command_index {
            Some(index) => {
                self.current_command_index = Some(index+1);
            },
            None => {
                self.current_command_index = Some(0);
            }
        }

        if self.has_more_commands() {
            let command_ref = &self.lines[self.current_command_index.unwrap()];
            self.current_command = Some(String::from(command_ref));
        }
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
        command.split("=").collect::<Vec<&str>>().first().unwrap().to_string()
    }

    fn comp(&self) -> String {
        let command = self.current_command.as_ref().unwrap();
        let components = command.split(";").collect::<Vec<&str>>();

        components[1].to_string()
    }

    fn jump(&self) -> String {
        let command = self.current_command.as_ref().unwrap();
        command.split(";").collect::<Vec<&str>>().last().unwrap().to_string()
    }
}
