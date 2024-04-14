use crate::command::Command;
use std::fs;

pub struct Parser {
    lines: Vec<String>,
    index: usize,
}

impl Parser {
    pub fn new(source_path: &String) -> Parser {
        let input = fs::read_to_string(source_path)
            .expect("Can't read input file");

        Parser {
            lines: input
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| {
                    !s.is_empty() && !s.starts_with("//")
                })
                .collect(),
            index: 0,
        }
    }
}

impl Iterator for Parser {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.lines.len() {
            return None;
        }
        let line = &self.lines[self.index];
        self.index += 1;

        match parse(line) {
            Some(c) => Some(c),
            None => self.next(),
        }
    }
}

pub fn parse(line: &str) -> Option<Command> {
    let result = match line.chars().next() {
        Some('@') => Some(parse_a_command(line)),
        Some(_) => Some(parse_c_command(line)),
        None => None,
    };
    result
}

fn parse_a_command(line: &str) -> Command {
    let address: String = line.chars().skip(1).collect();
    let address: u16 = match address.parse() {
        Ok(v) => v,
        Err(_) => todo!(),
    };
    Command::A { address }
}

fn parse_c_command(line: &str) -> Command {
    let mut dest: Option<String> = None;
    let mut comp: String = String::new();
    let mut jump: Option<String> = None;

    let mut buffer = String::new();
    for char in line.chars() {
        match char {
            '=' => {
                dest = Some(buffer);
                buffer = String::new();
            }
            ';' => {
                comp = buffer;
                buffer = String::new();
            }
            c => buffer.push(c),
        };
    }

    if comp.is_empty() {
        comp = buffer;
    } else {
        jump = Some(buffer);
    }

    Command::C { dest, comp, jump }
}
