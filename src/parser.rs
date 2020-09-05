use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
pub enum Command {
    ACommand,
    CCommand,
    LCommand,
}

pub struct Parser {
    lines: Vec<String>,
    current_command_index: Option<usize>,
    pub current_command: Option<String>,
}

pub fn new(filename: &str) -> Parser {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line);

            if line.starts_with("//") {
                continue;
            }

            if line.is_empty() {
                continue;
            }

            lines.push(line);
        }
    }

    Parser{
        lines,
        current_command_index: None,
        current_command: None,
    }
}

impl Parser {
    pub fn has_more_commands(&self) -> bool {
        match self.current_command_index {
            Some(index) => index < self.lines.len() - 1,
            None => true,
        }
    }

    pub fn advance(&mut self) {
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

    pub fn command_type(&self) -> Command {

        let command = self.current_command.as_ref().unwrap();
        if command.starts_with("@") {
            return Command::ACommand;
        }

        if command.contains("=") || command.contains(";") {
            return Command::CCommand;
        }

        Command::LCommand
    }

    pub fn symbol(&self) -> String {
        let command = self.current_command.as_ref().unwrap();
        match self.command_type() {
            Command::ACommand => command.trim_start_matches("@").to_string(),
            Command::LCommand => command.strip_prefix("(").unwrap().strip_suffix(")").unwrap().to_string(),
            _ => command.to_string(),
        }
    }

    pub fn dest(&self) -> Option<String> {
        let command = self.current_command.as_ref().unwrap();

        if command.contains("=") {
            return Some(command.split("=").collect::<Vec<&str>>().first().unwrap().to_string());
        }

        None
    }

    pub fn comp(&self) -> Option<String> {
        let mut command = self.current_command.as_ref().unwrap().clone();

        if command.contains("=") {
            command = command.split("=").collect::<Vec<&str>>().last().unwrap().to_string();
        }

        if command.contains(";") {
            command = command.split(";").collect::<Vec<&str>>().first().unwrap().to_string();
        }

        Some(command.to_string())
    }

    pub fn jump(&self) -> Option<String> {
        let command = self.current_command.as_ref().unwrap();

        if command.contains(";") {
            return Some(command.split(";").collect::<Vec<&str>>().last().unwrap().to_string());
        }

        None
    }
}    


