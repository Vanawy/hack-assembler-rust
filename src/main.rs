//! # Objective
//! Develop an assembler that translates programs written in the Hack assembly
//! language into Hack binary code. This version of the assembler assumes that
//! the source assembly code is valid. Error checking, reporting and handling
//! can be added to later versions of the assembler, but are not part of this
//! project.
//!
//! # Contract
//! When supplied to your assembler as a command-line argument, a Prog.asm file
//! containing a valid Hack assembly language program should be translated
//! correctly into Hack binary code, and stored in a file named Prog.hack,
//! located in the same folder as the source file (if a file by this name
//! exists, it is overridden). The output produced by your assembler must be
//! identical to the output produced by the supplied assembler.

use std::fs::File;
use std::io::Write;
use std::{env::args, fs};

mod code;
mod command;
mod parser;
mod symbol_table;

use code::Code;

use crate::command::{Command, PseudoCommand};
use crate::symbol_table::SymbolTable;

fn main() {
    let input_path = args().nth(1).expect(
        "Input .asm file path expected as first argument",
    );

    println!("Input: {}", input_path);

    let input = fs::read_to_string(&input_path)
        .expect("Can't read input file");

    let lines = input.lines();

    let lines = lines
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with("//"));

    let commands = lines.map(|s| {
        parser::parse(s)
            .take()
            .expect("Invalid string as parse input")
    });

    let mut symbol_table = SymbolTable::new();

    // FIRST PASS
    let mut rom_address = 0;

    commands.clone().for_each(|c| match c {
        Command::Pseudo(PseudoCommand::L { label }) => {
            symbol_table.insert(label, rom_address);
        }
        _ => {
            rom_address += 1;
        }
    });

    // SECOND PASS
    let mut variable_address = 16;

    let commands = commands.filter_map(|cmd| match cmd {
        Command::Pseudo(PseudoCommand::L { .. }) => None,
        Command::Pseudo(PseudoCommand::A { label }) => {
            if !symbol_table.has(&label) {
                symbol_table.insert(
                    label.clone(),
                    variable_address,
                );
                variable_address += 1;
            }
            Some(Command::A {
                address: symbol_table.get(&label).unwrap(),
            })
        }
        c => Some(c),
    });

    let codes: Vec<_> = commands.map(Code::from).collect();

    println!("{} commands parsed", codes.len());

    let name = input_path.replace(".asm", "") + ".hack";

    println!("Output: {}", name);

    // println!("Symbol table: {:?}", symbol_table);
    let mut f =
        File::create(name).expect("Error opening file");
    codes.iter().for_each(|c| {
        writeln!(f, "{}", c.to_binary_string())
            .expect("Error writing to file");
    });
}
