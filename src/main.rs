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

use std::{env::args, io::Write};

mod code;
mod command;
mod parser;

use code::Code;
use parser::Parser;
use std::fs::File;

fn main() {
    let input_path = args().nth(1).expect(
        "Input .asm file path expected as first argument",
    );

    println!("Input path: {}", input_path);

    let parser = Parser::new(&input_path);

    let codes: Vec<_> = parser.map(Code::from).collect();

    let name = input_path.replace(".asm", "") + ".hack";

    let mut f =
        File::create(name).expect("Error opening file");
    codes.iter().for_each(|c| {
        writeln!(f, "{}", c.to_binary_string())
            .expect("Error writing to file");
    });
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_max_l() {
        let p = |s: &str| -> String {
            let c = Code::from(parser::parse(s).unwrap());
            c.to_binary_string()
        };

        assert_eq!(p("@0"), "0000000000000000");
        assert_eq!(p("D=M"), "1111110000010000");
        assert_eq!(p("@1"), "0000000000000001");
        assert_eq!(p("D=D-M"), "1111010011010000");
        assert_eq!(p("@10"), "0000000000001010");
        assert_eq!(p("D;JGT"), "1110001100000001");
        assert_eq!(p("@1"), "0000000000000001");
        assert_eq!(p("D=M"), "1111110000010000");
        assert_eq!(p("@12"), "0000000000001100");
        assert_eq!(p("0;JMP"), "1110101010000111");
        assert_eq!(p("@0"), "0000000000000000");
        assert_eq!(p("D=M"), "1111110000010000");
        assert_eq!(p("@2"), "0000000000000010");
        assert_eq!(p("M=D"), "1110001100001000");
        assert_eq!(p("@14"), "0000000000001110");
        assert_eq!(p("0;JMP"), "1110101010000111");
    }
}
