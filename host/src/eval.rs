use std::collections::HashMap;

use crate::syntax::{Stmt, Token, Var};

#[derive(Debug, Default)]
pub struct State {
    vars: HashMap<Var, Vec<Token>>,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn get(&self, var: Var) -> Option<&Vec<Token>> {
        self.vars.get(&var)
    }

    pub fn eval(&mut self, stmt: Stmt) {
        
    }
}
