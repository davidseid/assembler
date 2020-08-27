
mod parser;
mod code;
use std::env;
use std::fs;
use std::io::prelude::*;



fn main() {
    println!("Assembler starting up...");

    let args: Vec<String> = env::args().collect();
    let assemblyFilename = &args[1];

    println!("Parsing assembly file: {}", assemblyFilename);
    let mut file_parser = parser::new(assemblyFilename);

    let fileNamePrefix = assemblyFilename.split(".").collect::<Vec<&str>>().first().unwrap().to_string();
    println!("{:?}", fileNamePrefix);

    println!("Opening binary file for writing: Add.hack");
    let mut hack_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("Add.hack")
        .unwrap();

    if let Err(e) = writeln!(hack_file, "some text") {
        eprintln!("Couldn't write to file: {}", e);
    }

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

    let code_bits = code::comp(Some(String::from("D|M")));
    println!("{}", code_bits);
}

