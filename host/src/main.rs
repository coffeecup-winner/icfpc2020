use std::{env, fs, io};

mod eval;
mod interact;
mod modem;
mod send;
mod syntax;
mod ui;

use crate::eval::{Picture, State};
use crate::syntax::*;
use crate::ui::ui_main;

fn print_pictures(pics: &[Picture]) {
    if pics.len() == 1 {
        println!("{}", pics[0]);
    } else {
        for (i, p) in pics.iter().enumerate() {
            println!("Picture #{}", i);
            println!("{}", p);
        }
    }
}

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
            let v = state.eval(&Var::Named("picture".to_string()));
            let pics = state.eval_picture_list(v);
            print_pictures(&pics);
        } else {
            let (expr, expected) = parse_test(line);
            state.interpret(expr);
            state.interpret(expected);
            assert_eq!(
                state.eval(&Var::Named("expr".to_string())),
                state.eval(&Var::Named("expected".to_string()))
            );
        }
    }
}

fn main() -> io::Result<()> {
    let file = fs::read_to_string(if env::args().len() == 2 {
        env::args().nth(1).unwrap()
    } else {
        "./data/i_stateless.txt".to_string()
    })?;
    if file.starts_with("TEST") {
        println!("Mode: test");
        run_test(file);
    } else if file.starts_with("INTERACTIVE") {
        println!("Mode: interactive");
        ui_main(file)?;
    } else {
        println!("Mode: custom");
        let mut state = State::new();
        for line in file.lines() {
            let stmt = parse_line(line);
            state.interpret(stmt);
        }
        println!("galaxy: {:?}", state.eval(&Var::Named("galaxy".to_string())));
    }
    Ok(())
}
