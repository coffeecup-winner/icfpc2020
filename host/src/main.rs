use std::{env, fs, io};

mod eval;
mod interact;
mod modem;
mod syntax;

use crate::eval::{BuiltIn, State, Value};
use crate::interact::*;
use crate::syntax::*;

fn run_test(file: String) {
    let mut state = State::new();
    // Skip the "TEST" line
    for line in file.lines().skip(1) {
        if line.is_empty() {
        } else if let Some(l) = line.strip_prefix("PRINT ") {
            println!("{}", l);
        } else if let Some(l) = line.strip_prefix("DRAW ") {
            let picture = parse_picture(l);
            state.interpret(picture);
            let v = state.eval(Var::Named("picture".to_string()));
            if let Value::Picture(p) = v {
                println!("{}", p);
            } else {
                let mut curr = v.clone();
                let mut i = 0;
                loop {
                    if let Value::Apply(f0, arg0) = curr {
                        if let Value::Apply(f1, arg1) = *f0 {
                            if let Value::BuiltIn(BuiltIn::Cons) = *f1 {
                                if let Value::Picture(p) = *arg1 {
                                    println!("Picture #{}:", i);
                                    i += 1;
                                    println!("{}", p);
                                    curr = *arg0;
                                    continue;
                                }
                            }
                        }
                    } else if Value::BuiltIn(BuiltIn::Nil) == curr {
                        break;
                    }
                    panic!("Not a picture: {:?}", v);
                }
            }
        } else {
            let (expr, expected) = parse_test(line);
            state.interpret(expr);
            state.interpret(expected);
            assert_eq!(
                state.eval(Var::Named("expr".to_string())),
                state.eval(Var::Named("expected".to_string()))
            );
        }
    }
}

fn main() -> io::Result<()> {
    if env::args().len() == 2 {
        let file = fs::read_to_string(env::args().nth(1).unwrap())?;
        if file.starts_with("TEST") {
            run_test(file);
        } else {
            let mut state = State::new();
            for line in file.lines() {
                let stmt = parse_line(line);
                state.interpret(stmt);
            }
            println!("galaxy: {:?}", state.eval(Var::Named("galaxy".to_string())));
        }
    } else {
        let mut state = State::new();
        state.interpret(parse_line("statelessdraw = ap ap c ap ap b b ap ap b ap b ap cons 0 ap ap c ap ap b b cons ap ap c cons nil ap ap c ap ap b cons ap ap c cons nil nil"));
        let p = run_interaction(state, "statelessdraw");
        println!("Result: {:#?}", p);
    }
    Ok(())
}
