use crate::syntax::*;
use crate::eval::*;

pub fn translate(text: String) -> String {
    let mut state = State::new();
    let mut vars = vec![];
    for line in text.lines() {
        let stmt = parse_line(line);
        vars.push(stmt.var.clone());
        state.interpret(stmt);
    }
    let mut result = String::new();
    result += "use crate::eval::*;\n";
    for var in vars {
        match &var {
            Var::Named(s) => {
                result += "pub fn ";
                result += s;
            }
            Var::Temp(v) => {
                result += "fn __";
                result += &format!("{}", v);
            }
        }
        result += "(v: Value) -> Value { ";
        // TODO
        result += "v";
        result += " }\n";
    }
    result
}
