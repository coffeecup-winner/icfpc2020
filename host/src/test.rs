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
}

impl Value {
    fn call(&self, arg: Rc<Value>) -> Result<Rc<Value>, ()> {
        match self {
            Value::Lambda(named_lambda) => Ok((named_lambda.lambda)(arg)),
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

fn test_bltin() -> Rc<Value> {
    Value::lambda("test", move |x| {
        Value::lambda("test'", move |y| {
            let x = match *x {
                Value::Number(n) => n,
                _ => panic!(""),
            };
            let y = match *y {
                Value::Number(n) => n,
                _ => panic!(""),
            };
            Value::number(x + y)
        })
    })
}

fn main() -> Result<(), ()>{
    let mut vars = HashMap::new();
    vars.insert(String::from("test"), test_bltin());

    let a = Value::number(12);
    let b = Value::number(30);

    let state = State { vars };

    let result_func = state.vars["test"].clone();
    println!("result: {:?}", result_func.call(a.clone())?.call(b.clone())?);

    Ok(())
}
