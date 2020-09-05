
mod parser;
mod code;
use std::env;
use std::fs;
use std::io::{Write, BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

fn main() {
    println!("Assembler starting up...");

    let args: Vec<String> = env::args().collect();
    let assembly_filename = &args[1];

    println!("Parsing assembly file: {}", assembly_filename);
    let mut file_parser = parser::new(assembly_filename);

    let filename_prefix = assembly_filename.split(".").collect::<Vec<&str>>().first().unwrap().to_string();

    let binary_filename = format!("{}.hack", filename_prefix);

    println!("Opening binary file for writing: {}", binary_filename);

    if Path::new(&binary_filename).exists() {
        fs::remove_file(&binary_filename);
    }

    let mut hack_file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(binary_filename)
        .unwrap();

    while file_parser.has_more_commands() {
        file_parser.advance();
        let command_type = file_parser.command_type();

        let command = &file_parser.current_command;
        println!("\n{:?}", command.as_ref().unwrap());

        println!("{:?}", command_type);

        match command_type {
            parser::Command::ACommand => {
                println!("A Command {}", file_parser.symbol());
                let binary = format!("{:016b}", file_parser.symbol().parse::<i32>().unwrap());
                writeln!(hack_file, "{}", &binary);
            },
            parser::Command::LCommand => {
                println!("Loop {}", file_parser.symbol());
            },
            parser::Command::CCommand => {
                println!("Dest {:?}", file_parser.dest());
                println!("Comp {:?}", file_parser.comp());
                println!("Jump {:?}", file_parser.jump());

                let comp_bits = code::comp(file_parser.comp());
                let dest_bits = code::dest(file_parser.dest());
                let jump_bits = code::jump(file_parser.jump());

                let binary = format!("111{}{}{}", comp_bits, dest_bits, jump_bits);
                println!("{}", binary);

                writeln!(hack_file, "{}", &binary);
            }
        }
    }

    hack_file.seek(SeekFrom::Start(0));
    let reader = BufReader::new(hack_file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
