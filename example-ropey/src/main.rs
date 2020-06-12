use std::fs::File;

use ropey::Rope;

fn main() {
    println!("Hello, world!");
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), std::io::Error> {
    let mut text = Rope::from_reader(File::open("a.txt")?)?;

    println!("{}", text.line(100));

    let start_idx = text.line_to_char(100);
    let end_idx = text.line_to_char(101);
    println!("{}", text.slice(start_idx..end_idx));

    Ok(())
}
