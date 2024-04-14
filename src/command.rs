#[derive(Debug)]
pub enum Command {
    /// A-command: `@value`
    /// Where value is either a non-negative decimal number
    A { address: u16 },
    /// C-command: `dest=comp;jump`
    /// Either the dest or jump fields may be empty.
    /// If dest is empty, the `=` is omitted;
    /// If jump is empty, the `;` is omitted.
    C {
        comp: String,
        dest: Option<String>,
        jump: Option<String>,
    },
    /// L-command (pseudo command): `@value`
    /// Where value is a symbol referring to number.
    L { label: String },
}
