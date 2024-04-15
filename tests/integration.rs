use std::{fs, path::PathBuf};

fn compare(filename: String) {
    let mut path: PathBuf =
        PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push("tests");
    path.push("resources");

    let asm = path.join(filename.clone() + ".asm");
    let hack =
        path.join(filename.clone() + ".expected.hack");

    let result = fs::read_to_string(hack).unwrap()
        == hack_asm::assemble(
            fs::read_to_string(asm).unwrap(),
        );
    assert!(result)
}

#[test]
fn test_add() {
    compare("Add".into());
}
#[test]
fn test_max() {
    compare("Max".into());
}
#[test]
fn test_rect() {
    compare("Rect".into());
}
#[test]
fn test_pong() {
    compare("Pong".into());
}
