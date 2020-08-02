pub enum Value {
    Number(i64),
    List(Vec<Value>),
    Func1(Func1),
    Func2(Func2),
}

impl Value {
    pub fn unwrap_number(self) -> i64 {
        if let Value::Number(n) = self {
            n
        } else {
            panic!("Failed to unwrap number");
        }
    }

    pub fn unwrap_list(self) -> Vec<Value> {
        if let Value::List(v) = self {
            v
        } else {
            panic!("Failed to unwrap list");
        }
    }
}

pub struct Func1(Box<dyn FnOnce(Value) -> Value>);
pub struct Func2(Box<dyn FnOnce(Value, Value) -> Value>);

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Number(v)
    }
}

impl From<Func1> for Value {
    fn from(f: Func1) -> Self {
        Value::Func1(f)
    }
}

impl From<Func2> for Value {
    fn from(f: Func2) -> Self {
        Value::Func2(f)
    }
}

pub fn inc() -> Value {
    Value::Number(0) // TODO
}

pub fn dec() -> Value {
    Value::Number(0) // TODO
}

pub fn add() -> Value {
    Value::Number(0) // TODO
}

pub fn mul() -> Value {
    Value::Number(0) // TODO
}

pub fn div() -> Value {
    Value::Number(0) // TODO
}

pub fn eq() -> Value {
    Value::Number(0) // TODO
}

pub fn lt() -> Value {
    Value::Number(0) // TODO
}

pub fn neg() -> Value {
    Value::Number(0) // TODO
}

pub fn s() -> Value {
    Value::Number(0) // TODO
}

pub fn c() -> Value {
    Value::Number(0) // TODO
}

pub fn b() -> Value {
    Value::Number(0) // TODO
}

pub fn t() -> Value {
    Value::Number(0) // TODO
}

pub fn f() -> Value {
    Value::Number(0) // TODO
}

pub fn pwr2() -> Value {
    Value::Number(0) // TODO
}

pub fn i() -> Value {
    Value::Number(0) // TODO
}

pub fn cons() -> Value {
    Value::Func2(Func2(Box::new(|a, b| _cons(a, b))))
}

pub fn head() -> Value {
    Value::Number(0) // TODO
}

pub fn tail() -> Value {
    Value::Number(0) // TODO
}

pub fn nil() -> Value {
    Value::List(vec![])
}

pub fn isnil() -> Value {
    Value::Number(0) // TODO
}

fn _cons(left: Value, right: Value) -> Value {
    match right {
        Value::List(mut l) => {
            l.push(left);
            Value::List(l)
        }
        _ => panic!("Trying to `cons` with a non-list value"),
    }
}

pub fn ap<T1: Into<Value>, T2: Into<Value>>(left: T1, right: T2) -> Value {
    match left.into() {
        Value::Func1(Func1(f1)) => f1(right.into()),
        Value::Func2(Func2(f2)) => {
            let v1 = right.into();
            Value::Func1(Func1(Box::new(move |v2| f2(v1, v2))))
        }
        _ => panic!("Trying to `ap` a non-function value"),
    }
}
