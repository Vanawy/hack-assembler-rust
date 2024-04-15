use crate::command::{Command, PseudoCommand};

pub fn parse(line: &str) -> Option<Command> {
    let result = match line.chars().next() {
        Some('@') => Some(parse_a_command(line)),
        Some('(') => Some(parse_l_definition(line)),
        Some(_) => Some(parse_c_command(line)),
        None => None,
    };
    result
}

fn parse_a_command(line: &str) -> Command {
    let chars: String = line.chars().skip(1).collect();
    match chars.parse() {
        Ok(address) => Command::A { address },
        Err(_) => Command::Pseudo(PseudoCommand::A {
            label: chars,
        }),
    }
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

fn parse_l_definition(line: &str) -> Command {
    let label = line
        .chars()
        .skip(1)
        .take_while(|&c| c != ')')
        .collect();

    Command::Pseudo(PseudoCommand::L { label })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_l_command() {
        assert!(matches!(
            parse_l_definition("(test)"),
            Command::Pseudo(PseudoCommand::L { label }) if label == "test",
        ));
    }
}
