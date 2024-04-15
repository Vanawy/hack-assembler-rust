use crate::command::Command;

#[derive(Debug)]
pub struct Code {
    value: u16,
}

impl Code {
    pub fn from(command: Command) -> Code {
        let value = match command {
            Command::A { address } => address,
            Command::C { .. } => {
                value_from_c_command(command)
            }
            Command::Pseudo(_) => panic!(
                "Pseudo-command cant be converted to code"
            ),
        };

        Code { value }
    }

    pub fn to_binary_string(&self) -> String {
        format!("{:016b}", self.value)
    }
}

fn value_from_c_command(command: Command) -> u16 {
    let mut value = 0b111 << 13;

    match command {
        Command::C { comp, dest, jump } => {
            add_comp_part(&mut value, comp);

            if let Some(dest) = dest {
                add_dest_part(&mut value, dest);
            }

            if let Some(jump) = jump {
                add_jump_part(&mut value, jump);
            }
        }
        _ => panic!("C-command variant expected"),
    };

    value
}

fn add_comp_part(value: &mut u16, comp: String) {
    let comp: u16 = match &comp[..] {
        // a = 0
        "0" => 0b0_101010,
        "1" => 0b0_111111,
        "-1" => 0b0_111010,
        "D" => 0b0_001100,
        "A" => 0b0_110000,
        "!D" => 0b0_001101,
        "!A" => 0b0_110001,
        "-D" => 0b0_001111,
        "-A" => 0b0_110011,
        "D+1" => 0b0_011111,
        "A+1" => 0b0_110111,
        "D-1" => 0b0_001110,
        "A-1" => 0b0_110010,
        "D+A" => 0b0_000010,
        "D-A" => 0b0_010011,
        "A-D" => 0b0_000111,
        "D&A" => 0b0_000000,
        "D|A" => 0b0_010101,
        // a = 1
        "M" => 0b1_110000,
        "!M" => 0b1_110001,
        "-M" => 0b1_110011,
        "M+1" => 0b1_110111,
        "M-1" => 0b1_110010,
        "D+M" => 0b1_000010,
        "D-M" => 0b1_010011,
        "M-D" => 0b1_000111,
        "D&M" => 0b1_000000,
        "D|M" => 0b1_010101,
        // default
        _ => panic!("Invalid comp part of C-command"),
    };

    *value |= comp << 6;
}

fn add_dest_part(value: &mut u16, dest: String) {
    let dest: u16 = match &dest[..] {
        "M" => 1,
        "D" => 2,
        "MD" => 3,
        "A" => 4,
        "AM" => 5,
        "AD" => 6,
        "AMD" => 7,
        _ => panic!("Invalid dest part of C-command"),
    };

    *value |= dest << 3;
}
fn add_jump_part(value: &mut u16, comp: String) {
    let jump: u16 = match &comp[..] {
        "JGT" => 1,
        "JEQ" => 2,
        "JGE" => 3,
        "JLT" => 4,
        "JNE" => 5,
        "JLE" => 6,
        "JMP" => 7,
        _ => panic!("Invalid jump part of C-command"),
    };

    *value |= jump;
}
