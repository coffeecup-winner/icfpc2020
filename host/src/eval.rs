use std::collections::HashMap;

use crate::syntax::{Stmt, Token, Var};

#[derive(Debug, Default)]
pub struct State {
    raw: HashMap<Var, Vec<Token>>,
    compiled: HashMap<Var, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Var(u32),
    Number(i64),
    Bool(bool),
    List(Vec<Value>), // stored in reverse
    BuiltIn(BuiltIn),
    Func(Vec<Token>),
    Partial(Box<Value>, Box<Value>),
}

impl Value {
    pub fn arity(&self) -> u32 {
        use Value::*;
        match self {
            Var(_) => panic!(),
            Number(_) => 0,
            List(_) => 0,
            Bool(_) => 0,
            BuiltIn(b) => b.arity(),
            Func(_) => panic!(),
            Partial(v, _) => v.arity() - 1,
        }
    }
}

// Built-in functions except `ap`
#[derive(Debug, Clone)]
pub enum BuiltIn {
    Inc,   // #5
    Dec,   // #6
    Add,   // #7
    Mul,   // #9
    Div,   // #10
    Eq,    // #11
    Lt,    // #12
    Mod,   // #13 - ???
    Dem,   // #14 - ???
    Send,  // #15 - ???
    Neg,   // #16
    S,     // #18
    C,     // #19
    B,     // #20
    Pwr2,  // #23 - ???
    I,     // #24
    Cons,  // #25
    Head,  // #26
    Tail,  // #27
    Nil,   // #28
    IsNil, // #29
    // #30 - ???
    // #31 - ???
    Draw,          // #32
    Checkerboard,  // #33
    MultiDraw,     // #34
    ModList,       // #35 - ???
    Send2,         // #36 - ???
    If0,           // #37
    Interact,      // #38-39 - ???
    StatelessDraw, // #40 - ???
    StatefulDraw,  // #41 - ???
    Galaxy,        // #42
}

impl BuiltIn {
    pub fn arity(&self) -> u32 {
        use BuiltIn::*;
        match self {
            Inc => 1,
            Dec => 1,
            Add => 2,
            Mul => 2,
            Div => 2,
            Eq => 2,
            Lt => 2,
            Mod => 1,
            Dem => 1,
            Send => panic!(),
            Neg => 1,
            S => 3,
            C => 3,
            B => 3,
            True => 0,
            False => 0,
            Pwr2 => 1,
            I => 1,
            Cons => 2, // doesn't exactly match the definition
            Head => 1,
            Tail => 1,
            Nil => 0, // doesn't exactly match the definition
            IsNil => 1,
            // #30 - ???
            // #31 - ???
            Draw => panic!(),
            Checkerboard => panic!(),
            MultiDraw => panic!(),
            ModList => panic!(),
            Send2 => panic!(),
            If0 => panic!(),
            Interact => panic!(),
            StatelessDraw => panic!(),
            StatefulDraw => panic!(),
            Galaxy => panic!(),
        }
    }
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn exec(&mut self, var: Var) -> Value {
        let v = self.eval(var).clone();
        self.eval_value(v)
    }

    pub fn eval(&mut self, var: Var) -> &Value {
        println!("{:?}", var);
        if self.raw.get(&var).is_some() {
            let code = self.raw.remove(&var).unwrap();
            // First precompile all dependencies
            for t in code.iter() {
                if let Token::Var(v) = t {
                    if self.raw.get(&Var::Temp(*v)).is_some() {
                        self.eval(Var::Temp(*v));
                    }
                }
            }

            let v = self.compile(code);
            self.compiled.insert(var.clone(), v);
        }
        self.compiled.get(&var).unwrap()
    }

    pub fn eval_value(&mut self, val: Value) -> Value {
        match val {
            Value::Var(v) => {
                let v = self.eval(Var::Temp(v)).clone();
                self.eval_value(v)
            },
            Value::Number(_) => val,
            Value::Bool(_) => val,
            Value::List(_) => val,
            Value::BuiltIn(_) => val,
            Value::Func(_) => val,
            Value::Partial(_, _) => val,
        }
    }

    pub fn interpret(&mut self, stmt: Stmt) {
        self.raw.insert(stmt.var, stmt.code);
    }

    fn compile(&self, code: Vec<Token>) -> Value {
        let mut stack: Vec<Value> = vec![];
        for token in code.into_iter().rev() {
            match token {
                Token::Var(v) => stack.push(Value::Var(v)),

                Token::Number(n) => stack.push(Value::Number(n)),
                Token::True => stack.push(Value::Bool(true)),
                Token::False => stack.push(Value::Bool(false)),
                Token::Nil => stack.push(Value::List(vec![])),

                Token::Inc => stack.push(Value::BuiltIn(BuiltIn::Inc)),
                Token::Dec => stack.push(Value::BuiltIn(BuiltIn::Dec)),
                Token::Add => stack.push(Value::BuiltIn(BuiltIn::Add)),
                Token::Mul => stack.push(Value::BuiltIn(BuiltIn::Mul)),
                Token::Div => stack.push(Value::BuiltIn(BuiltIn::Div)),
                Token::Eq => stack.push(Value::BuiltIn(BuiltIn::Eq)),
                Token::Lt => stack.push(Value::BuiltIn(BuiltIn::Lt)),
                Token::Mod => stack.push(Value::BuiltIn(BuiltIn::Mod)),
                Token::Dem => stack.push(Value::BuiltIn(BuiltIn::Dem)),
                Token::Send => stack.push(Value::BuiltIn(BuiltIn::Send)),
                Token::Neg => stack.push(Value::BuiltIn(BuiltIn::Neg)),
                Token::S => stack.push(Value::BuiltIn(BuiltIn::S)),
                Token::C => stack.push(Value::BuiltIn(BuiltIn::C)),
                Token::B => stack.push(Value::BuiltIn(BuiltIn::B)),
                Token::Pwr2 => stack.push(Value::BuiltIn(BuiltIn::Pwr2)),
                Token::I => stack.push(Value::BuiltIn(BuiltIn::I)),
                Token::Cons => stack.push(Value::BuiltIn(BuiltIn::Cons)),
                Token::Head => stack.push(Value::BuiltIn(BuiltIn::Head)),
                Token::Tail => stack.push(Value::BuiltIn(BuiltIn::Tail)),
                Token::IsNil => stack.push(Value::BuiltIn(BuiltIn::IsNil)),
                Token::Draw => stack.push(Value::BuiltIn(BuiltIn::Draw)),
                Token::Checkerboard => stack.push(Value::BuiltIn(BuiltIn::Checkerboard)),
                Token::MultiDraw => stack.push(Value::BuiltIn(BuiltIn::MultiDraw)),
                Token::ModList => stack.push(Value::BuiltIn(BuiltIn::ModList)),
                Token::Send2 => stack.push(Value::BuiltIn(BuiltIn::Send2)),
                Token::If0 => stack.push(Value::BuiltIn(BuiltIn::If0)),
                Token::Interact => stack.push(Value::BuiltIn(BuiltIn::Interact)),
                Token::StatelessDraw => stack.push(Value::BuiltIn(BuiltIn::StatelessDraw)),
                Token::StatefulDraw => stack.push(Value::BuiltIn(BuiltIn::StatefulDraw)),
                Token::Galaxy => stack.push(Value::BuiltIn(BuiltIn::Galaxy)),

                Token::Ap => match stack.pop().unwrap() {
                    p @ Value::Partial(_, _) => {
                        match p.arity() {
                            0 => panic!("Illegal state"),
                            1 => {
                                // Applying partially applied functions
                                if let Value::Partial(f, arg) = p {
                                    match *f {
                                        Value::BuiltIn(BuiltIn::Cons) => {
                                            let head = arg;
                                            if let Value::List(mut tail) = stack.pop().unwrap() {
                                                tail.push(*head);
                                                stack.push(Value::List(tail));
                                            } else {
                                                panic!("Invalid arguments for `cons`");
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
                            }
                            _ => {
                                let v = stack.pop().unwrap();
                                stack.push(Value::Partial(Box::new(p), Box::new(v)));
                            }
                        }
                    }
                    Value::BuiltIn(b) => {
                        match b.arity() {
                            0 => panic!("Illegal state: {:?}", b),
                            1 => {
                                // apply function with arity 1
                                match b {
                                    BuiltIn::Inc => panic!(),
                                    BuiltIn::Dec => panic!(),
                                    BuiltIn::Mod => panic!(),
                                    BuiltIn::Dem => panic!(),
                                    BuiltIn::Neg => {
                                        let v = stack.pop().unwrap();
                                        if let Value::Number(v) = v {
                                            stack.push(Value::Number(-v));
                                        } else {
                                            panic!("Invalid argument for `neg`")
                                        }
                                    }
                                    BuiltIn::Pwr2 => panic!(),
                                    BuiltIn::I => panic!(),
                                    BuiltIn::Head => panic!(),
                                    BuiltIn::Tail => panic!(),
                                    BuiltIn::IsNil => panic!(),
                                    _ => panic!("Invalid function: {:?}", b),
                                }
                            }
                            _ => {
                                // function with arity > 1, partially apply
                                let v = stack.pop().unwrap();
                                stack
                                    .push(Value::Partial(Box::new(Value::BuiltIn(b)), Box::new(v)));
                            }
                        }
                    }
                    f => panic!("Unsupported function: {:?}", f),
                },
                _ => panic!("{:?}", token),
            }
        }
        stack[0].clone()
    }
}
