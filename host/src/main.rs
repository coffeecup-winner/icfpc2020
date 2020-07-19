use std::{env, fs, io, thread};

mod eval;
mod interact;
mod modem;
mod send;
mod syntax;
mod types;
mod ui;

use crate::eval::State;
use crate::syntax::*;
use crate::types::*;
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
            let v = state.eval_v(&Var::Named("picture".to_string()));
            let list = NestedList::from_value(v);
            let pics = PictureBuilder::from_nested_list(list);
            print_pictures(&pics);
        } else {
            let (expr, expected) = parse_test(line);
            state.interpret(expr);
            state.interpret(expected);
            assert_eq!(
                state.eval_v(&Var::Named("expr".to_string())),
                state.eval_v(&Var::Named("expected".to_string()))
            );
        }
    }
}

fn run() -> io::Result<()> {
    let path = if env::args().len() == 2 {
        env::args().nth(1).unwrap()
    } else {
        "./data/i_stateless.txt".to_string()
    };
    let data_folder = std::path::Path::new(&path).parent().unwrap();
    let file = fs::read_to_string(&path)?;
    if file.starts_with("TEST") {
        println!("Mode: test");
        run_test(file);
    } else if file.starts_with("INTERACTIVE") {
        println!("Mode: interactive");
        ui_main(file, &data_folder)?;
    } else {
        println!("Mode: custom");
        let mut state = State::new();
        for line in file.lines() {
            let stmt = parse_line(line);
            state.interpret(stmt);
        }
        println!(
            "galaxy: {:?}",
            state.eval_v(&Var::Named("galaxy".to_string()))
        );
    }
    Ok(())
}

// Remove later if not needed
const THREAD_STACK_SIZE: usize = 16 * 1024 * 1024;

fn main() -> io::Result<()> {
    let child_thread = thread::Builder::new()
        .stack_size(THREAD_STACK_SIZE)
        .spawn(run)?;

    child_thread.join().unwrap()
}
