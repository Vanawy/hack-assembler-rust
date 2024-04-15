mod code;
mod command;
mod parser;
mod symbol_table;

use crate::command::{Command, PseudoCommand};
use crate::symbol_table::SymbolTable;

use crate::code::Code;

pub fn assemble(input: String) -> String {
    let lines: Vec<String> = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with("//"))
        .map(|s| s.into())
        .collect();

    let commands: Vec<Command> = parse_text(lines);

    let mut symbol_table = SymbolTable::new();

    first_pass(&commands, &mut symbol_table);

    let commands = second_pass(commands, &mut symbol_table);

    let codes: Vec<_> =
        commands.into_iter().map(Code::from).collect();

    println!("{} commands parsed", codes.len());

    codes
        .iter()
        .map(|c| c.to_binary_string() + "\n")
        .collect()
}

fn parse_text(lines: Vec<String>) -> Vec<Command> {
    lines
        .iter()
        .map(|s| {
            parser::parse(s)
                .take()
                .expect("Invalid string as parse input")
        })
        .collect()
}

/// Inserti all labels (xxx) into symbol table with corresponding addresses
fn first_pass(
    commands: &[Command],
    symbol_table: &mut SymbolTable,
) {
    let mut rom_address = 0;

    commands.iter().for_each(|c| match c {
        Command::Pseudo(PseudoCommand::L { label }) => {
            symbol_table
                .insert(label.to_string(), rom_address);
        }
        _ => {
            rom_address += 1;
        }
    });
}
/// Remove all pseudo instructions
///
/// Replace `@var` with actual adressing instruction
fn second_pass(
    commands: Vec<Command>,
    symbol_table: &mut SymbolTable,
) -> Vec<Command> {
    let mut variable_address = 16;

    commands
        .into_iter()
        .filter_map(|cmd| match cmd {
            Command::Pseudo(PseudoCommand::L {
                ..
            }) => None,
            Command::Pseudo(PseudoCommand::A { label }) => {
                if !symbol_table.has(&label) {
                    symbol_table.insert(
                        label.clone(),
                        variable_address,
                    );
                    variable_address += 1;
                }
                Some(Command::A {
                    address: symbol_table
                        .get(&label)
                        .expect("Variable address was not found in symbol table"),
                })
            }
            c => Some(c.clone()),
        })
        .collect()
}
