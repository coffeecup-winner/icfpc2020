use crate::eval::*;
use crate::syntax::*;

// fn f38(protocol: Var, new_state: Var, data: Var) {}

fn interact(state: &mut State, var_protocol: Var, var_state: Var, var_vector: Var) {
    let var_result = Var::Named("__result".to_string());
    // TODO: write __flag, __new_state and __data
    state.interpret(Stmt {
        var: var_result.clone(),
        code: vec![
            // Token::Ap,
            // Token::Head,
            // Token::Ap,
            // Token::Tail,
            // Token::Ap,
            // Token::Tail,
            Token::Ap,
            Token::Ap,
            Token::Var(var_protocol.clone()),
            Token::Var(var_state),
            Token::Var(var_vector),
        ],
    });
    let val = state.eval(var_result);
    println!("#1: {:#?}", val);
    // f38(protocol, )
}

pub fn run_interaction(mut state: State, protocol: &str) {
    let var_protocol = Var::Named(protocol.to_string());
    let var_state = Var::Named("__state".to_string());
    let var_vector = Var::Named("__vector".to_string());
    state.interpret(Stmt {
        var: var_state.clone(),
        code: vec![Token::Nil],
    });
    state.interpret(Stmt {
        var: var_vector.clone(),
        code: vec![
            Token::Ap,
            Token::Ap,
            Token::Cons,
            Token::Number(1),
            Token::Number(0),
        ],
    });
    interact(&mut state, var_protocol, var_state, var_vector);
}
