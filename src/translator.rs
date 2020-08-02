use std::collections::HashMap;

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
        translate_named_value(&mut result, &var, &state.get(&var), true);
    }
    result
}

fn translate_named_value(result: &mut String, var: &Var, value: &Value, split: bool) {
    if split && get_value_max_depth(value) > 1000 {
        let parts = split_value(var, value);
        // TODO: sort by var
        for (var, val) in parts {
            println!("DEPTH: {}", get_value_max_depth(&val));
            translate_named_value(result, &var, &val, false);
        }
    } else {
        match &var {
            Var::Named(s) => {
                result.push_str("pub fn ");
                result.push_str(s);
            }
            Var::Temp(v) => {
                // TODO: remove `pub` when done
                result.push_str("pub fn __");
                result.push_str(&format!("{}", v));
            }
        }
        result.push_str("() -> Value { ");
        translate_value(result, var, value);
        result.push_str(" }\n");
    }
}

fn translate_value(result: &mut String, var: &Var, value: &Value) {
    match &value.borrow().val {
        Value_::Var(v) => match v {
            Var::Named(s) => {
                result.push_str(&s);
                result.push_str("()");
            }
            Var::Temp(v) => {
                result.push_str("__");
                result.push_str(&format!("{}()", v));
            }
        },
        Value_::Number(n) => result.push_str(&format!("Value::Number({})", n)),
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
            translate_value(result, var, left);
            result.push_str(", ");
            translate_value(result, var, right);
            result.push_str(")");
        }
    }
}

fn split_value(var: &Var, value: &Value) -> HashMap<Var, Value> {
    let base_name = match var {
        Var::Named(s) => format!("{}_", s),
        Var::Temp(v) => format!("__{}_", v),
    };
    let mut name_gen = NameGen::new(base_name);
    let mut parts = HashMap::new();
    let mut var = var.clone();
    let mut value = value.clone();
    loop {
        let mut more = None;
        let new_value = split_value_recursive(value, 1, &mut name_gen, &mut more);
        parts.insert(var, new_value);
        if let Some((more_var, more_value)) = more {
            var = more_var;
            value = more_value;
        } else {
            break;
        }
    }
    parts
}

struct NameGen {
    base: String,
    idx: u32,
}

impl NameGen {
    pub fn new(base: String) -> Self {
        NameGen { base, idx: 0 }
    }

    pub fn next(&mut self) -> String {
        let mut result = self.base.clone();
        result += &format!("{}", self.idx);
        self.idx += 1;
        result
    }
}

fn split_value_recursive(
    value: Value,
    current_depth: u32,
    name_gen: &mut NameGen,
    more: &mut Option<(Var, Value)>,
) -> Value {
    match &value.borrow().val {
        Value_::Var(v) => var(v.clone()),
        Value_::Number(n) => number(*n),
        Value_::BuiltIn(f) => b(*f),
        Value_::Apply(left, right) => {
            if current_depth + get_value_max_depth(left) < 1000 {
                ap(
                    left.clone(),
                    split_value_recursive(right.clone(), current_depth + 1, name_gen, more),
                )
            } else {
                let v = Var::Named(name_gen.next());
                *more = Some((v.clone(), right.clone()));
                ap(left.clone(), var(v))
            }
        }
    }
}

fn get_value_max_depth(value: &Value) -> u32 {
    // TODO: cache this if needed
    match &value.borrow().val {
        Value_::Var(_) => 1,
        Value_::Number(_) => 1,
        Value_::BuiltIn(_) => 1,
        Value_::Apply(left, right) => get_value_max_depth(left).max(get_value_max_depth(right)) + 1,
    }
}
