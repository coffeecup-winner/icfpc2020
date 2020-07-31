use crate::eval::*;
use crate::syntax::*;

pub fn translate(text: String) -> String {
    let mut state = State::new();
    let mut vars = vec![];
    for line in text.lines() {
        let stmt = parse_line(line);
        vars.push(stmt.var.clone());
        state.interpret(stmt);
    }
    let mut result = String::new();
    result += "use crate::runtime::*;\n";
    for var in vars {
        match &var {
            Var::Named(s) => {
                result += "pub fn ";
                result += s;
            }
            Var::Temp(v) => {
                // TODO: remove `pub` when done
                result += "pub fn __";
                result += &format!("{}", v);
            }
        }
        result += "(v: Value) -> Value { ";
        translate_value(&mut result, &state.get(&var));
        result += " }\n";
        break;
    }
    result
}

fn translate_value(result: &mut String, value: &Value) {
    match &value.borrow().val {
        Value_::Var(v) => match v {
            Var::Named(s) => result.push_str(&s),
            Var::Temp(v) => {
                result.push_str("__");
                result.push_str(&format!("{}", v));
            }
        },
        Value_::Number(n) => result.push_str(&format!("{}", n)),
        Value_::BuiltIn(b) => {
            result.push_str(match b {
                BuiltIn::Inc => "inc",
                BuiltIn::Dec => "dec",
                BuiltIn::Add => "add",
                BuiltIn::Mul => "mul",
                BuiltIn::Div => "div",
                BuiltIn::Eq => "eq",
                BuiltIn::Lt => "lt",
                BuiltIn::Neg => "neg",
                BuiltIn::S => "s",
                BuiltIn::C => "c",
                BuiltIn::B => "b",
                BuiltIn::True => "t",
                BuiltIn::False => "f",
                BuiltIn::Pwr2 => "pwr2",
                BuiltIn::I => "i",
                BuiltIn::Cons => "cons",
                BuiltIn::Head => "head",
                BuiltIn::Tail => "tail",
                BuiltIn::Nil => "nil",
                BuiltIn::IsNil => "isnil",
            });
            result.push_str("()");
        }
        Value_::Apply(left, right) => {
            result.push_str("ap(");
            translate_value(result, left);
            result.push_str(", ");
            translate_value(result, right);
            result.push_str(")");
        }
    }
}
