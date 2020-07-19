use crate::eval::*;
use crate::syntax::*;

pub fn run_interaction(state: &mut State, protocol: &str, x: i64, y: i64) -> Vec<Picture> {
    let var_protocol = Var::Named(protocol.to_string());
    let var_result = Var::Named("__result".to_string());
    let var_state = Var::Named("__state".to_string());
    if !state.contains(&var_state) {
        state.insert(var_state.clone(), b(BuiltIn::Nil));
    }
    state.interpret(Stmt {
        var: var_result.clone(),
        code: vec![
            Token::Ap,
            Token::Ap,
            Token::Ap,
            Token::Interact,
            Token::Var(var_protocol),
            Token::Var(var_state.clone()),
            Token::Ap,
            Token::Ap,
            Token::Cons,
            Token::Number(x),
            Token::Number(y),
        ],
    });
    state.eval_v(&var_result);

    let var_picture = Var::Named("__picture".to_string());
    state.interpret(Stmt {
        var: var_picture.clone(),
        code: vec![
            Token::Ap,
            Token::Head,
            Token::Ap,
            Token::Tail,
            Token::Var(var_result.clone()),
        ],
    });
    let v = state.eval_v(&var_picture);
    let pics = state.eval_picture_list(v);

    let var_new_state = Var::Named("__new_state".to_string());
    state.interpret(Stmt {
        var: var_new_state.clone(),
        code: vec![Token::Ap, Token::Head, Token::Var(var_result)],
    });
    let v = state.eval_v(&var_new_state);
    state.insert(var_state, v);

    pics
}
