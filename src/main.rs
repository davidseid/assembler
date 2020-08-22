
mod parser;
mod code;
use std::env;

fn main() {
    println!("Assembler starting up...");

    let args: Vec<String> = env::args().collect();

    println!("Parsing assembly file: {}", &args[1]);
    let mut file_parser = parser::new(&args[1]);

    while file_parser.has_more_commands() {
        file_parser.advance();
        let command_type = file_parser.command_type();

        let command = &file_parser.current_command;
        println!("\n{:?}", command.as_ref().unwrap());

        println!("{:?}", command_type);

        match command_type {
            parser::Command::ACommand | parser::Command::LCommand => println!("Symbol {}", file_parser.symbol()),
            parser::Command::CCommand => {
                println!("Dest {:?}", file_parser.dest());
                println!("Comp {:?}", file_parser.comp());
                println!("Jump {:?}", file_parser.jump());
            }
        }
    }

    code::comp(Some(String::from("0")));
}

