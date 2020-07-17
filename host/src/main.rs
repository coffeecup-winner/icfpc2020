use std::{env, fs, io};

mod syntax;

use crate::syntax::*;

fn main() -> io::Result<()> {
    let file = fs::read_to_string(env::args().nth(1).unwrap())?;
    for line in file.lines() {
        let stmt = parse_line(line);
        // println!("{:?}", stmt);
        // return Ok(());
    }
    Ok(())
}
