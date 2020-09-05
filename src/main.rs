
mod parser;
mod code;
mod symbol;
use std::env;
use std::fs;
use std::io::{Write, BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

fn main() {
    println!("Assembler starting up...");

    let args: Vec<String> = env::args().collect();
    let assembly_filename = &args[1];

    println!("Parsing assembly file: {}", assembly_filename);

    let mut first_pass_parser = parser::new(assembly_filename);
    let mut file_parser = parser::new(assembly_filename);

    let filename_prefix = assembly_filename.split(".").collect::<Vec<&str>>().first().unwrap().to_string();

    let binary_filename = format!("{}.hack", filename_prefix);

    if Path::new(&binary_filename).exists() {
        fs::remove_file(&binary_filename);
    }

    let mut hack_file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(&binary_filename)
        .unwrap();

    let mut symbol_table = symbol::new();
    symbol_table.add_predefined();

    let mut rom_address = 0;

    while first_pass_parser.has_more_commands() {
        first_pass_parser.advance();
        let command_type = first_pass_parser.command_type();

        match command_type {
            parser::Command::ACommand => rom_address+=1,
            parser::Command::LCommand => symbol_table.add_entry(first_pass_parser.symbol(), rom_address),
            parser::Command::CCommand => rom_address+=1,
        }
    }
    
    let mut ram_address = 16;

    while file_parser.has_more_commands() {
        file_parser.advance();
        let command_type = file_parser.command_type();

        match command_type {
            parser::Command::ACommand => {

                let mut symbol = file_parser.symbol();
                let symbol_ref = &symbol;

                if !symbol_ref.chars().all(char::is_numeric) {

                    if !symbol_table.contains(String::from(symbol_ref)) {
                        symbol_table.add_entry(String::from(symbol_ref), ram_address);
                        ram_address += 1;
                    }
                    let value = symbol_table.get_address(String::from(symbol_ref)).to_string();
                    symbol = value;
                } 

                let binary = format!("{:016b}", symbol.parse::<i32>().unwrap());
                println!("Writing line {}, from symbol {}", &binary, &symbol);
                writeln!(hack_file, "{}", &binary);

            },
            parser::Command::LCommand => {},
            parser::Command::CCommand => {
                let dest = file_parser.dest();
                let comp = file_parser.comp();
                let jump = file_parser.jump();
                let comp_bits = code::comp(comp);
                let dest_bits = code::dest(dest);
                let jump_bits = code::jump(jump);

                let binary = format!("111{}{}{}", comp_bits, dest_bits, jump_bits);
                // println!("Writing line {}, from {:?}{:?}{:?}", &binary, file_parser.dest().unwrap(), file_parser.comp().unwrap(), file_parser.jump().unwrap());
                println!("Writing line {}", &binary);
                println!("From Command: {}", file_parser.current_command.as_ref().unwrap());
                println!("Comp bits {}", &comp_bits);
                println!("Dest bits {}", &dest_bits);
                println!("Jump bits {}", &jump_bits);

                writeln!(hack_file, "{}", &binary);
            }
        }
    }

    hack_file.seek(SeekFrom::Start(0));
    let reader = BufReader::new(hack_file);

    println!("\n\n{} file written...", &binary_filename);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
