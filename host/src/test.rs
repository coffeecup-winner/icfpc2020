use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

type VarMap = HashMap<String, Rc<Value>>;

#[derive(Debug, Default)]
struct State {
    vars: VarMap,
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
    Apply(Rc<Value>, Rc<Value>),
}

impl Value {
    fn call(&self, state: &mut State, arg: Rc<Value>) -> Result<Rc<Value>, ()> {
        match self {
            Value::Lambda(named_lambda) => Ok((named_lambda.lambda)(arg, state)),
            // callins a cons is like applying something to the pair
            Value::Cons(a, b) => Ok(Value::apply(Value::apply(arg, a.clone()), b.clone())),
            _ => Err(()),
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
        return Rc::new(Value::Apply(f, x));
    }

    fn atom(atom: String) -> Rc<Value> {
        return Rc::new(Value::Atom(atom));
    }

    fn nil() -> Rc<Value> {
        return Rc::new(Value::Nil);
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

fn builtin_cons() -> Rc<Value> {
    Value::lambda("cons", move |a, _| {
        Value::lambda("cons'", move |b, _| Value::cons(a.clone(), b.clone()))
    })
}

fn builtin_true() -> Rc<Value> {
    Value::lambda("true", move |a, _| {
        Value::lambda("true'", move |_b, _| a.clone())
    })
}

fn builtin_false() -> Rc<Value> {
    Value::lambda("false", move |_a, _| {
        Value::lambda("false'", move |b, _| b.clone())
    })
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

// use std::{env, fs, io};
// use std::fs::File;
// use std::io::BufReader;
// use std::path::Path;

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

    println!("{:?}", nesting_stack);

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

fn main() -> Result<(), ()> {
    let mut vars = HashMap::new();
    vars.insert(String::from("cons"), builtin_cons());
    vars.insert(String::from("t"), builtin_true());
    vars.insert(String::from("f"), builtin_false());

    let a = Value::number(12);
    let b = Value::number(30);

    let mut state = State { vars };

    let result_func = state.vars["cons"].clone();
    println!(
        "result: {:?}",
        result_func
            .call(&mut state, a.clone())?
            .call(&mut state, b.clone())?
    );

    let parsed = parse_line("ap cons ( 1 , 2 , ( 3 ) , 4 )", &state.vars)?;
    println!("{:?}", parsed);

    Ok(())
}
