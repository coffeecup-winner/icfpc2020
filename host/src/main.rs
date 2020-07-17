use std::{env, fs, io};

mod eval;
mod syntax;

use crate::eval::State;
use crate::syntax::*;

fn main() -> io::Result<()> {
    let file = fs::read_to_string(env::args().nth(1).unwrap())?;
    let mut state = State::new();
    for line in file.lines() {
        let stmt = parse_line(line);
        state.eval(stmt);
    }
    println!("galaxy: {:?}", state.get(Var::Named("galaxy".to_string())));
    Ok(())
}
