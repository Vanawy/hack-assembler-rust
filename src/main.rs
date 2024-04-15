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

use std::{env::args, fs};

fn main() {
    let input_path = args().nth(1).expect(
        "Input .asm file path expected as first argument",
    );

    println!("Input: {}", input_path);

    let input = fs::read_to_string(&input_path)
        .expect("Can't read input file");

    let output = hack_asm::assemble(input);

    let output_path =
        input_path.replace(".asm", "") + ".hack";

    println!("Output: {}", output_path);

    fs::write(output_path, output)
        .expect("Can't write output to file");
}
