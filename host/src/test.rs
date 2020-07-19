use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Default)]
struct State {
    vars: HashMap<String, Rc<Value>>,
}

#[derive(Debug)]
enum Value {
    Lambda(NamedLambda),
    Number(usize),
    Nil,
    True,
    False,
    Cons(Rc<Value>, Rc<Value>),
    Apply(Rc<Value>, Rc<Value>),
}

impl Value {
    fn call(&self, arg: Rc<Value>) -> Result<Rc<Value>, ()> {
        match self {
            Value::Lambda(named_lambda) => Ok((named_lambda.lambda)(arg)),
            // callins a cons is like applying something to the pair
            Value::Cons(a, b) => Ok(Value::apply(Value::apply(arg, a.clone()), b.clone())),
            _ => Err(()),
        }
    }

    fn lambda<T: 'static>(name: &'static str, lambda: T) -> Rc<Value>
    where
        T: Fn(Rc<Value>) -> Rc<Value>,
    {
        Rc::new(Value::Lambda(NamedLambda {
            name: name,
            lambda: Box::new(lambda),
        }))
    }

    fn number(number: usize) -> Rc<Value> {
        return Rc::new(Value::Number(number));
    }

    fn cons(a: Rc<Value>, b: Rc<Value>) -> Rc<Value> {
        return Rc::new(Value::Cons(a, b));
    }

    fn apply(f: Rc<Value>, x: Rc<Value>) -> Rc<Value> {
        return Rc::new(Value::Apply(f, x));
    }
}

type LambdaBox = Box<dyn Fn(Rc<Value>) -> Rc<Value>>;
struct NamedLambda {
    lambda: LambdaBox,
    name: &'static str,
}

impl fmt::Debug for NamedLambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NamedLambda")
            .field("name", &self.name)
            .finish()
    }
}

fn builtin_cons() -> Rc<Value> {
    Value::lambda("cons", move |a| {
        Value::lambda("cons'", move |b| Value::cons(a.clone(), b.clone()))
    })
}

fn builtin_true() -> Rc<Value> {
    Value::lambda("true", move |a| {
        Value::lambda("true'", move |_b| a.clone())
    })
}

fn builtin_false() -> Rc<Value> {
    Value::lambda("false", move |_a| {
        Value::lambda("false'", move |b| b.clone())
    })
}

fn main() -> Result<(), ()> {
    let mut vars = HashMap::new();
    vars.insert(String::from("cons"), builtin_cons());
    vars.insert(String::from("t"), builtin_true());
    vars.insert(String::from("f"), builtin_false());

    let a = Value::number(12);
    let b = Value::number(30);

    let state = State { vars };

    let result_func = state.vars["test"].clone();
    println!(
        "result: {:?}",
        result_func.call(a.clone())?.call(b.clone())?
    );

    Ok(())
}
