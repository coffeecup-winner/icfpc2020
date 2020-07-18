use crate::eval::*;
use crate::syntax::*;

pub fn run_interaction(mut state: State, protocol: &str) -> Value {
    let var_protocol = Var::Named(protocol.to_string());
    let var_result = Var::Named("__result".to_string());
    state.interpret(Stmt {
        var: var_result.clone(),
        code: vec![
            Token::Ap,
            Token::Ap,
            Token::Ap,
            Token::Interact,
            Token::Var(var_protocol),
            Token::Nil,
            Token::Ap,
            Token::Ap,
            Token::Cons,
            Token::Number(1),
            Token::Number(0),
        ],
    });
    state.eval(var_result.clone());
    let var_picture = Var::Named("__picture".to_string());
    state.interpret(Stmt {
        var: var_picture.clone(),
        code: vec![
            Token::Ap,
            Token::Head,
            Token::Ap,
            Token::Tail,
            Token::Var(var_result),
        ],
    });
    state.eval(var_picture.clone())
}
