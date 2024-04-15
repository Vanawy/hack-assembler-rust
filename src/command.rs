#[derive(Debug, Clone)]
pub enum Command {
    /// A-command: `@value`
    /// Where value is either a non-negative decimal number [AAddress] or a
    /// symbol [ALabel] referring to such number
    A {
        address: u16,
    },
    /// C-command: `dest=comp;jump`
    /// Either the dest or jump fields may be empty.
    /// If dest is empty, the `=` is omitted;
    /// If jump is empty, the `;` is omitted.
    C {
        comp: String,
        dest: Option<String>,
        jump: Option<String>,
    },
    Pseudo(PseudoCommand),
}

#[derive(Debug, Clone)]
pub enum PseudoCommand {
    /// A-command (pseudo command): `@label`
    ///
    /// variable
    A { label: String },
    /// L-command (pseudo command): `(label)`
    ///
    L { label: String },
}
