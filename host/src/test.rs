use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

type VarMap = HashMap<String, Rc<Value>>;

#[derive(Debug)]
struct State {
    vars: VarMap,
}

#[derive(Debug)]
struct Application {
    cache: RefCell<Option<Rc<Value>>>,
    f: Rc<Value>,
    x: Rc<Value>,
}

impl Application {
    fn eval_uncached(&self, state: &mut State) -> Rc<Value> {
        let f = Value::eval(&self.f, state);
        f.call(state, self.x.clone())
    }

    fn eval(&self, state: &mut State) -> Rc<Value> {
        self.cache.borrow_mut()
            .get_or_insert_with(|| self.eval_uncached(state))
            .clone()
    }
}


#[derive(Debug)]
enum Value {
    Lambda(NamedLambda),
    Atom(String),
    Number(i64),
    Nil,
    True,
    False,
    Cons(Rc<Value>, Rc<Value>),
    Application(Application)
}

impl Value {
    fn call(&self, state: &mut State, arg: Rc<Value>) -> Rc<Value> {
        match self {
            Value::Application(appl) => Value::call(&appl.eval(state), state, arg),
            Value::Atom(name) => match state.vars.get(name) {
                Some(rc) => rc.clone().call(state, arg),
                None => panic!("no such atom: {}", name),
            },
            Value::Lambda(named_lambda) => (named_lambda.lambda)(arg, state),
            // callins a cons is like applying something to the pair
            Value::Cons(a, b) => Value::apply(Value::apply(arg, a.clone()), b.clone()),
            // ap nil x0   =   t
            Value::Nil => Value::make_true(),
            // ap ap t x0 x1   =   x0
            Value::True => Value::lambda("true'", move |_b, _| arg.clone()),
            // ap ap f x0 x1   =   x1
            Value::False => Value::lambda("false'", move |b, _| b.clone()),
            _ => panic!("tried to call {:?}", self),
        }
    }

    fn eval(rc: &Rc<Value>, state: &mut State) -> Rc<Value> {
        match &**rc {
            Value::Application(appl) => appl.eval(state),
            _ => rc.clone(),
        }
    }

    fn eval_number(rc: &Rc<Value>, state: &mut State) -> i64 {
        let val = Value::eval(rc, state);
        match *val {
            Value::Number(n) => n,
            _ => panic!("evaluated as a number: {:#?}", val),
        }
    }

    fn lambda<T: 'static>(name: &'static str, lambda: T) -> Rc<Value>
    where
        T: Fn(Rc<Value>, &mut State) -> Rc<Value>,
    {
        Rc::new(Value::Lambda(NamedLambda {
            name: name,
            lambda: Box::new(lambda),
        }))
    }

    fn number(number: i64) -> Rc<Value> {
        return Rc::new(Value::Number(number));
    }

    fn cons(a: Rc<Value>, b: Rc<Value>) -> Rc<Value> {
        return Rc::new(Value::Cons(a, b));
    }

    fn apply(f: Rc<Value>, x: Rc<Value>) -> Rc<Value> {
        return Rc::new(Value::Application(Application { cache: RefCell::new(None), f, x }));
    }

    fn atom(atom: String) -> Rc<Value> {
        return Rc::new(Value::Atom(atom));
    }

    fn nil() -> Rc<Value> {
        return Rc::new(Value::Nil);
    }

    fn make_true() -> Rc<Value> {
        return Rc::new(Value::True);
    }

    fn make_false() -> Rc<Value> {
        return Rc::new(Value::False);
    }

    fn make_bool(val: bool) -> Rc<Value> {
        if val {
            Value::make_true()
        } else {
            Value::make_false()
        }
    }
}

type LambdaBox = Box<dyn Fn(Rc<Value>, &mut State) -> Rc<Value>>;
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

pub enum Token<'a> {
    Atom(&'a str),
    Number(i64),
    Assign,
    ListOpen,
    ListSep,
    ListClose,
}

struct Lexer<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    string_iter: T,
}

impl<'a, T> Lexer<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    fn from(string_iter: T) -> Lexer<'a, T> {
        Lexer { string_iter }
    }
}

impl<'a, T> Iterator for Lexer<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        use Token::*;
        let string = self.string_iter.next()?;
        Some(match string {
            "(" => ListOpen,
            ")" => ListClose,
            "," => ListSep,
            "=" => Assign,
            string => {
                if let Ok(number) = string.parse::<i64>() {
                    Number(number)
                } else {
                    Atom(string)
                }
            }
        })
    }
}

#[derive(Debug)]
struct ParseResult {
    name: Option<String>,
    ast: Rc<Value>,
}

fn parse_line(line: &str, vars: &VarMap) -> Result<ParseResult, ()> {
    use Token::*;

    let mut var_name: Option<String> = None;
    let mut line_tokens = vec![];

    let mut lexer = Lexer::from(line.split(' '));

    // push the first two tokens
    for _ in 0..2 {
        match lexer.next() {
            None => break,
            Some(e) => line_tokens.push(e),
        }
    }

    // check for name = ..
    match line_tokens.as_slice() {
        [Atom(name), Assign] => {
            var_name = Some(String::from(*name));
            line_tokens.clear();
        }
        _ => (),
    }

    // lex the end of the line
    for token in lexer {
        line_tokens.push(token);
    }

    if line_tokens.len() == 0 {
        return Err(());
    }

    // there's a stack of values for each nested list level
    let mut nesting_stack: Vec<Vec<Rc<Value>>> = vec![];
    nesting_stack.push(vec![]);

    for token in line_tokens.into_iter().rev() {
        let stack = nesting_stack.last_mut().unwrap();

        use Token::*;
        match token {
            Assign => panic!("unexpected = token in the middle of the line"),
            Atom("ap") => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Value::apply(a, b));
            }
            Atom(name) => {
                if let Some(var) = vars.get(name) {
                    stack.push(var.clone())
                } else {
                    stack.push(Value::atom(String::from(name)))
                }
            }
            Number(n) => stack.push(Value::number(n)),
            ListClose => nesting_stack.push(vec![Value::nil()]),
            ListSep => {
                let cons = match stack.as_slice() {
                    [tail, head] => Value::cons(head.clone(), tail.clone()),
                    _ => panic!(),
                };
                stack.clear();
                stack.push(cons)
            }
            ListOpen => {
                let cons = match stack.as_slice() {
                    [tail, head] => Value::cons(head.clone(), tail.clone()),
                    _ => panic!(),
                };
                nesting_stack.pop();
                nesting_stack.last_mut().unwrap().push(cons);
            }
        }
    }

    assert!(nesting_stack.len() == 1);
    let top_stack = &nesting_stack[0];
    match top_stack.as_slice() {
        [e] => Ok(ParseResult {
            name: var_name,
            ast: e.clone(),
        }),
        _ => panic!(),
    }
}

fn builtin_cons() -> Rc<Value> {
    Value::lambda("cons", move |a, _| {
        Value::lambda("cons'", move |b, _| Value::cons(a.clone(), b.clone()))
    })
}

fn builtin_car(true_impl: Rc<Value>) -> Rc<Value> {
    Value::lambda("car", move |elem, _| {
        Value::apply(true_impl.clone(), elem.clone())
    })
}

fn builtin_cdr(false_impl: Rc<Value>) -> Rc<Value> {
    Value::lambda("cdr", move |elem, _| {
        Value::apply(false_impl.clone(), elem.clone())
    })
}

fn builtin_isnil(true_impl: Rc<Value>, false_impl: Rc<Value>) -> Rc<Value> {
    Value::lambda("isnil", move |elem, _| {
        match *elem {
            Value::Nil => true_impl.clone(),
            _ => false_impl.clone(),
        }
    })
}

fn builtin_i() -> Rc<Value> {
    Value::lambda("i", move |elem, _| {
        elem.clone()
    })
}

fn builtin_inc() -> Rc<Value> {
    Value::lambda("inc", move |elem, state| {
        Value::number(Value::eval_number(&elem, state) + 1)
    })
}

fn builtin_dec() -> Rc<Value> {
    Value::lambda("dec", move |elem, state| {
        Value::number(Value::eval_number(&elem, state) - 1)
    })
}

fn builtin_pwr2() -> Rc<Value> {
    Value::lambda("pwr2", move |elem, state| {
        Value::number(1i64 << Value::eval_number(&elem, state))
    })
}

fn builtin_add() -> Rc<Value> {
    Value::lambda("add", move |a, _| {
        Value::lambda("add'", move |b, state| {
            let a = Value::eval_number(&a, state);
            let b = Value::eval_number(&b, state);
            Value::number(a + b)
        })
    })
}

fn builtin_mul() -> Rc<Value> {
    Value::lambda("mul", move |a, _| {
        Value::lambda("mul'", move |b, state| {
            let a = Value::eval_number(&a, state);
            let b = Value::eval_number(&b, state);
            Value::number(a * b)
        })
    })
}

fn builtin_div() -> Rc<Value> {
    Value::lambda("div", move |a, _| {
        Value::lambda("div'", move |b, state| {
            let a = Value::eval_number(&a, state);
            let b = Value::eval_number(&b, state);
            Value::number(a / b)
        })
    })
}

fn builtin_eq() -> Rc<Value> {
    Value::lambda("eq", move |a, _| {
        Value::lambda("eq'", move |b, state| {
            let a = Value::eval_number(&a, state);
            let b = Value::eval_number(&b, state);
            Value::make_bool(a == b)
        })
    })
}

fn builtin_lt() -> Rc<Value> {
    Value::lambda("lt", move |a, _| {
        Value::lambda("lt'", move |b, state| {
            let a = Value::eval_number(&a, state);
            let b = Value::eval_number(&b, state);
            Value::make_bool(a < b)
        })
    })
}

fn builtin_neg() -> Rc<Value> {
    Value::lambda("neg", move |arg, state| {
        Value::number(-Value::eval_number(&arg, state))
    })
}

fn builtin_s() -> Rc<Value> {
    Value::lambda("s", move |a, _| {
        Value::lambda("s'", move |b, _| {
            let a = a.clone();
            Value::lambda("s''", move |c, _| {
                let left = Value::apply(a.clone(), c.clone());
                let right = Value::apply(b.clone(), c.clone());
                Value::apply(left, right)
            })
        })
    })
}

fn builtin_c() -> Rc<Value> {
    Value::lambda("c", move |a, _| {
        Value::lambda("c'", move |b, _| {
            let a = a.clone();
            Value::lambda("c''", move |c, _| {
                Value::apply(Value::apply(a.clone(), c.clone()), b.clone())
            })
        })
    })
}

fn builtin_b() -> Rc<Value> {
    Value::lambda("b", move |a, _| {
        Value::lambda("b'", move |b, _| {
            let a = a.clone();
            Value::lambda("b''", move |c, _| {
                Value::apply(b.clone(), Value::apply(a.clone(), c.clone()))
            })
        })
    })
}

use std::{env, fs, io};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::BufRead;

fn main() -> Result<(), ()> {
    let mut vars = HashMap::new();
    let builtin_true_value = Value::make_true();
    let builtin_false_value = Value::make_false();
    vars.insert(String::from("cons"), builtin_cons());
    vars.insert(String::from("t"), builtin_true_value.clone());
    vars.insert(String::from("f"), builtin_false_value.clone());
    vars.insert(String::from("car"), builtin_car(builtin_true_value.clone()));
    vars.insert(String::from("cdr"), builtin_cdr(builtin_false_value.clone()));
    vars.insert(String::from("nil"), Value::nil());
    vars.insert(String::from("isnil"), builtin_isnil(builtin_true_value.clone(), builtin_false_value.clone()));
    vars.insert(String::from("i"), builtin_i());
    vars.insert(String::from("inc"), builtin_inc());
    vars.insert(String::from("dec"), builtin_dec());
    vars.insert(String::from("add"), builtin_add());
    vars.insert(String::from("mul"), builtin_mul());
    vars.insert(String::from("div"), builtin_div());
    vars.insert(String::from("eq"), builtin_eq());
    vars.insert(String::from("lt"), builtin_lt());
    vars.insert(String::from("neg"), builtin_neg());
    vars.insert(String::from("s"), builtin_s());
    vars.insert(String::from("c"), builtin_c());
    vars.insert(String::from("b"), builtin_b());
    vars.insert(String::from("pwr2"), builtin_pwr2());

    let mut state = State { vars };

    let file = match File::open("../data/galaxy.txt") {
        Ok(f) => f,
        Err(_) => panic!("failed to open file"),
    };

    let file = BufReader::new(file);

    for line in file.lines().map(|l| l.unwrap()) {
        let parsed = parse_line(&line, &state.vars)?;
        if let Some(var_name) = parsed.name {
            state.vars.insert(var_name, parsed.ast.clone());
        }
    }

    let galaxy = state.vars["galaxy"].clone();

    // make a call to galaxy that gets the first image
    let zero_zero = Value::cons(Value::number(0), Value::number(0));
    let galaxy_appl = Value::apply(Value::apply(galaxy.clone(), Value::nil()), zero_zero);

    println!("{:?}", Value::eval(&galaxy_appl, &mut state));

    Ok(())
}
