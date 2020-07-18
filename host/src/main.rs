use std::{env, fs, io};

mod eval;
mod syntax;

use crate::eval::State;
use crate::syntax::*;

fn main() -> io::Result<()> {
    let file = fs::read_to_string(env::args().nth(1).unwrap())?;
    if file.starts_with("TEST") {
        let mut state = State::new();
        // Skip the "TEST" line
        for line in file.lines().skip(1) {
            if line.is_empty() {
                continue;
            } else if let Some(l) = line.strip_prefix("PRINT ") {
                println!("{}", l);
                continue;
            }
            let (expr, expected) = parse_test(line);
            state.interpret(expr);
            state.interpret(expected);
            assert_eq!(
                state.eval(Var::Named("expr".to_string())),
                state.eval(Var::Named("expected".to_string()))
            );
        }
    } else {
        let mut state = State::new();
        for line in file.lines() {
            let stmt = parse_line(line);
            state.interpret(stmt);
        }
        println!("galaxy: {:?}", state.eval(Var::Named("galaxy".to_string())));
    }
    Ok(())
}
