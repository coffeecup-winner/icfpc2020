use std::collections::HashMap;

use crate::modem::{self, NestedList};
use crate::send;
use crate::syntax::{Stmt, Token, Var};

#[derive(Debug, Default)]
pub struct State {
    vars: HashMap<Var, Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Var(Var),
    Number(i64),
    Signal(Vec<bool>), // used with modulate / demodulate
    Picture(Picture),
    BuiltIn(BuiltIn),
    Apply(Box<Value>, Box<Value>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub points: Vec<Point>,
}

impl Picture {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, x: u32, y: u32) {
        // TODO: maybe calculate these later if slow
        if x >= self.width {
            self.width = x + 1;
        }
        if y >= self.height {
            self.height = y + 1;
        }
        self.points.push(Point { x, y });
    }
}

impl std::fmt::Display for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // SLOW - TODO REWRITE
        for y in 0..self.height {
            for x in 0..self.width {
                if self.points.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            if y != self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

// Built-in functions except `ap`
#[derive(Debug, PartialEq, Clone)]
pub enum BuiltIn {
    Inc,       // #5
    Dec,       // #6
    Add,       // #7
    Mul,       // #9
    Div,       // #10
    Eq,        // #11
    Lt,        // #12
    Mod,       // #13
    Dem,       // #14
    Send,      // #15
    Neg,       // #16
    S,         // #18
    C,         // #19
    B,         // #20
    True,      // #21
    False,     // #22
    Pwr2,      // #23
    I,         // #24
    Cons,      // #25
    Head,      // #26
    Tail,      // #27
    Nil,       // #28
    IsNil,     // #29
    Draw,      // #32
    MultiDraw, // #34
    If0,       // #37
    Interact,  // #38-39
    F38,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn contains(&self, var: &Var) -> bool {
        self.vars.contains_key(var)
    }

    pub fn insert(&mut self, var: Var, val: Value) {
        self.vars.insert(var, val);
    }

    pub fn eval(&self, var: &Var) -> Value {
        let v = self.vars.get(var).unwrap().clone();
        self.eval_value(v, true)
    }

    pub fn eval_deep(&self, var: &Var) -> Value {
        let v = self.vars.get(var).unwrap().clone();
        self.eval_value(v, false)
    }

    // This will put a single picture into a vector as well
    pub fn eval_picture_list(&self, val: Value) -> Vec<Picture> {
        if let Value::Picture(p) = val {
            return vec![p];
        } else {
            let mut curr = val.clone();
            let mut result = vec![];
            loop {
                if let Value::Apply(f0, tail) = curr {
                    if let Value::Apply(f1, head) = *f0 {
                        if let Value::BuiltIn(BuiltIn::Cons) = *f1 {
                            if let Value::Picture(p) = *head {
                                result.push(p);
                                curr = *tail;
                                continue;
                            }
                        }
                    }
                } else if Value::BuiltIn(BuiltIn::Nil) == curr {
                    break;
                }
                panic!("Not a picture: {:?}", val);
            }
            result
        }
    }

    fn eval_value(&self, val: Value, lazy: bool) -> Value {
        // println!("eval_value: {:?}", val);
        match val.clone() {
            Value::Var(v) => self.eval_value(self.vars.get(&v).unwrap().clone(), lazy),
            Value::Number(_) => val,
            Value::Signal(_) => val,
            Value::Picture(_) => val,
            Value::BuiltIn(_) => val,
            Value::Apply(f0, arg0) => {
                match self.eval_value(*f0, false) {
                    Value::BuiltIn(BuiltIn::Send) => {
                        let arg0 = self.eval_nested_list(*arg0);
                        let signal = modem::mod_list(&arg0);
                        let signal_str = signal
                            .into_iter()
                            .map(|x| if x { '1' } else { '0' })
                            .collect();
                        let endpoint =
                            String::from("https://icfpc2020-api.testkontur.ru/aliens/send");
                        let token = std::env::var("ICFPC_TEAM_TOKEN").ok();
                        let response = match send::request(&endpoint, token, &signal_str) {
                            Ok(val) => val,
                            Err(err) => panic!("request failed: {:?}", err),
                        };

                        let demodulated =
                            match modem::demodulate(&mut response.chars().map(|x| x != '0')) {
                                Ok(val) => val,
                                Err(err) => panic!("demodulation failed: {:?}", err),
                            };
                        println!("demodulated to: {:?}", demodulated);

                        Value::Number(1)
                    }
                    Value::BuiltIn(BuiltIn::Inc) => {
                        if let Value::Number(n) = self.eval_value(*arg0, lazy) {
                            Value::Number(n + 1)
                        } else {
                            panic!("Invalid argument for `inc`");
                        }
                    }
                    Value::BuiltIn(BuiltIn::Dec) => {
                        if let Value::Number(n) = self.eval_value(*arg0, lazy) {
                            Value::Number(n - 1)
                        } else {
                            panic!("Invalid argument for `dec`");
                        }
                    }
                    Value::BuiltIn(BuiltIn::Mod) => Value::Signal(modem::mod_list(
                        &self.eval_nested_list(self.eval_value(*arg0, false)),
                    )),
                    Value::BuiltIn(BuiltIn::Dem) => {
                        if let Value::Signal(s) = self.eval_value(*arg0, lazy) {
                            let list = modem::dem_list(&s);
                            panic!("TODO: demodulate {:?}", list);
                        } else {
                            panic!("Invalid argument for `dem`");
                        }
                    }
                    Value::BuiltIn(BuiltIn::Neg) => {
                        if let Value::Number(n) = self.eval_value(*arg0, lazy) {
                            Value::Number(-n)
                        } else {
                            panic!("Invalid argument for `neg`");
                        }
                    }
                    Value::BuiltIn(BuiltIn::Pwr2) => {
                        if let Value::Number(n) = self.eval_value(*arg0, lazy) {
                            Value::Number((2 as i64).pow(n as u32))
                        } else {
                            panic!("Invalid argument for `pwr2`");
                        }
                    }
                    Value::BuiltIn(BuiltIn::I) => self.eval_value(*arg0, lazy),
                    Value::BuiltIn(BuiltIn::Head) => {
                        let e_arg0 = if lazy {
                            arg0
                        } else {
                            Box::new(self.eval_value(*arg0, lazy))
                        };
                        self.eval_value(
                            Value::Apply(e_arg0, Box::new(Value::BuiltIn(BuiltIn::True))),
                            lazy,
                        )
                    }
                    Value::BuiltIn(BuiltIn::Tail) => {
                        let e_arg0 = if lazy {
                            arg0
                        } else {
                            Box::new(self.eval_value(*arg0, lazy))
                        };
                        self.eval_value(
                            Value::Apply(e_arg0, Box::new(Value::BuiltIn(BuiltIn::False))),
                            lazy,
                        )
                    }
                    Value::BuiltIn(BuiltIn::Nil) => Value::BuiltIn(BuiltIn::True),
                    Value::BuiltIn(BuiltIn::IsNil) => {
                        if let Value::BuiltIn(BuiltIn::Nil) = *arg0 {
                            Value::BuiltIn(BuiltIn::True)
                        } else {
                            Value::BuiltIn(BuiltIn::False)
                        }
                    }
                    Value::BuiltIn(BuiltIn::Draw) => Value::Picture(self.eval_draw(*arg0)),
                    Value::BuiltIn(BuiltIn::MultiDraw) => self.eval_multidraw(*arg0),

                    // ===== Arity 2 =====
                    Value::Apply(f1, arg1) => {
                        match self.eval_value(*f1, false) {
                            Value::BuiltIn(BuiltIn::Add) => {
                                if let Value::Number(b) = self.eval_value(*arg0, lazy) {
                                    if let Value::Number(a) = self.eval_value(*arg1, lazy) {
                                        Value::Number(a + b)
                                    } else {
                                        panic!("Invalid argument for `add`");
                                    }
                                } else {
                                    panic!("Invalid argument for `add`");
                                }
                            }
                            Value::BuiltIn(BuiltIn::Mul) => {
                                if let Value::Number(b) = self.eval_value(*arg0, lazy) {
                                    if let Value::Number(a) = self.eval_value(*arg1, lazy) {
                                        Value::Number(a * b)
                                    } else {
                                        panic!("Invalid argument for `mul`");
                                    }
                                } else {
                                    panic!("Invalid argument for `mul`");
                                }
                            }
                            Value::BuiltIn(BuiltIn::Div) => {
                                if let Value::Number(b) = self.eval_value(*arg0, lazy) {
                                    if let Value::Number(a) = self.eval_value(*arg1, lazy) {
                                        Value::Number(a / b)
                                    } else {
                                        panic!("Invalid argument for `div`");
                                    }
                                } else {
                                    panic!("Invalid argument for `div`");
                                }
                            }
                            Value::BuiltIn(BuiltIn::Eq) => {
                                if let Value::Number(b) = self.eval_value(*arg0, lazy) {
                                    if let Value::Number(a) = self.eval_value(*arg1, lazy) {
                                        if a == b {
                                            Value::BuiltIn(BuiltIn::True)
                                        } else {
                                            Value::BuiltIn(BuiltIn::False)
                                        }
                                    } else {
                                        panic!("Invalid argument for `eq`");
                                    }
                                } else {
                                    panic!("Invalid argument for `eq`");
                                }
                            }
                            Value::BuiltIn(BuiltIn::Lt) => {
                                if let Value::Number(b) = self.eval_value(*arg0, lazy) {
                                    if let Value::Number(a) = self.eval_value(*arg1, lazy) {
                                        if a < b {
                                            Value::BuiltIn(BuiltIn::True)
                                        } else {
                                            Value::BuiltIn(BuiltIn::False)
                                        }
                                    } else {
                                        panic!("Invalid argument for `lt`");
                                    }
                                } else {
                                    panic!("Invalid argument for `lt`");
                                }
                            }
                            Value::BuiltIn(BuiltIn::True) => self.eval_value(*arg1, lazy),
                            Value::BuiltIn(BuiltIn::False) => self.eval_value(*arg0, lazy),
                            Value::BuiltIn(BuiltIn::F38) => {
                                let e_arg0 = Box::new(self.eval_value(*arg1, lazy));
                                let e_arg1 = Box::new(self.eval_value(*arg0, lazy));
                                self.eval_value(Self::construct_f38_builtin(e_arg0, e_arg1), lazy)
                            }

                            // ===== Arity 3 =====
                            Value::Apply(f2, arg2) => {
                                match self.eval_value(*f2, false) {
                                    Value::BuiltIn(BuiltIn::S) => {
                                        let (e_arg0, e_arg1, e_arg2) = if lazy {
                                            (arg2, arg1, arg0)
                                        } else {
                                            (
                                                Box::new(self.eval_value(*arg2, lazy)),
                                                Box::new(self.eval_value(*arg1, lazy)),
                                                Box::new(self.eval_value(*arg0, lazy)),
                                            )
                                        };
                                        let (e_ap0, e_ap1) = if lazy {
                                            (
                                                Value::Apply(e_arg0, e_arg2.clone()), // If costly, use Rc
                                                Value::Apply(e_arg1, e_arg2),
                                            )
                                        } else {
                                            (
                                                self.eval_value(
                                                    Value::Apply(e_arg0, e_arg2.clone()),
                                                    lazy,
                                                ), // If costly, use Rc
                                                self.eval_value(Value::Apply(e_arg1, e_arg2), lazy),
                                            )
                                        };
                                        self.eval_value(
                                            Value::Apply(Box::new(e_ap0), Box::new(e_ap1)),
                                            lazy,
                                        )
                                    }
                                    Value::BuiltIn(BuiltIn::C) => {
                                        let (e_arg0, e_arg1, e_arg2) = if lazy {
                                            (arg2, arg1, arg0)
                                        } else {
                                            (
                                                Box::new(self.eval_value(*arg2, lazy)),
                                                Box::new(self.eval_value(*arg1, lazy)),
                                                Box::new(self.eval_value(*arg0, lazy)),
                                            )
                                        };
                                        let e_ap0 = if lazy {
                                            Value::Apply(e_arg0, e_arg2)
                                        } else {
                                            self.eval_value(Value::Apply(e_arg0, e_arg2), lazy)
                                        };
                                        self.eval_value(Value::Apply(Box::new(e_ap0), e_arg1), lazy)
                                    }
                                    Value::BuiltIn(BuiltIn::B) => {
                                        let (e_arg0, e_arg1, e_arg2) = if lazy {
                                            (arg2, arg1, arg0)
                                        } else {
                                            (
                                                Box::new(self.eval_value(*arg2, lazy)),
                                                Box::new(self.eval_value(*arg1, lazy)),
                                                Box::new(self.eval_value(*arg0, lazy)),
                                            )
                                        };
                                        let e_ap0 = if lazy {
                                            Value::Apply(e_arg1, e_arg2)
                                        } else {
                                            self.eval_value(Value::Apply(e_arg1, e_arg2), lazy)
                                        };
                                        self.eval_value(Value::Apply(e_arg0, Box::new(e_ap0)), lazy)
                                    }
                                    Value::BuiltIn(BuiltIn::Cons) => {
                                        let (e_arg0, e_arg1, e_arg2) = if lazy {
                                            (arg2, arg1, arg0)
                                        } else {
                                            (
                                                Box::new(self.eval_value(*arg2, lazy)),
                                                Box::new(self.eval_value(*arg1, lazy)),
                                                Box::new(self.eval_value(*arg0, lazy)),
                                            )
                                        };
                                        let e_ap0 = if lazy {
                                            Value::Apply(e_arg2, e_arg0)
                                        } else {
                                            self.eval_value(Value::Apply(e_arg2, e_arg0), lazy)
                                        };
                                        self.eval_value(Value::Apply(Box::new(e_ap0), e_arg1), lazy)
                                    }
                                    Value::BuiltIn(BuiltIn::If0) => {
                                        if let Value::Number(0) = self.eval_value(*arg2, lazy) {
                                            self.eval_value(*arg1, lazy)
                                        } else {
                                            self.eval_value(*arg0, lazy)
                                        }
                                    }
                                    Value::BuiltIn(BuiltIn::Interact) => {
                                        let (e_arg0, e_arg1, e_arg2) = if lazy {
                                            (arg2, arg1, arg0)
                                        } else {
                                            (
                                                Box::new(self.eval_value(*arg2, lazy)),
                                                Box::new(self.eval_value(*arg1, lazy)),
                                                Box::new(self.eval_value(*arg0, lazy)),
                                            )
                                        };
                                        let (e_ap0, e_ap1) = if lazy {
                                            (
                                                Value::Apply(
                                                    Box::new(Value::BuiltIn(BuiltIn::F38)),
                                                    e_arg0.clone(),
                                                ),
                                                Value::Apply(
                                                    Box::new(Value::Apply(e_arg0, e_arg1)),
                                                    e_arg2,
                                                ),
                                            )
                                        } else {
                                            (
                                                self.eval_value(
                                                    Value::Apply(
                                                        Box::new(Value::BuiltIn(BuiltIn::F38)),
                                                        e_arg0.clone(),
                                                    ),
                                                    lazy,
                                                ),
                                                self.eval_value(
                                                    Value::Apply(
                                                        Box::new(self.eval_value(
                                                            Value::Apply(e_arg0, e_arg1),
                                                            lazy,
                                                        )),
                                                        e_arg2,
                                                    ),
                                                    lazy,
                                                ),
                                            )
                                        };
                                        self.eval_value(
                                            Value::Apply(Box::new(e_ap0), Box::new(e_ap1)),
                                            lazy,
                                        )
                                    }
                                    _ => val,
                                }
                            }
                            _ => val,
                        }
                    }
                    _ => val,
                }
            }
        }
    }

    fn construct_f38_builtin(arg0: Box<Value>, arg1: Box<Value>) -> Value {
        use self::BuiltIn::*;
        use self::Value::*;
        let b = |x| Box::new(x);
        Apply(
            b(Apply(
                b(Apply(
                    b(BuiltIn(If0)),
                    b(Apply(b(BuiltIn(Head)), arg1.clone())),
                )),
                b(Apply(
                    b(Apply(
                        b(BuiltIn(Cons)),
                        b(Apply(
                            b(BuiltIn(I)), // TODO: modem
                            b(Apply(
                                b(BuiltIn(Head)),
                                b(Apply(b(BuiltIn(Tail)), arg1.clone())),
                            )),
                        )),
                    )),
                    b(Apply(
                        b(Apply(
                            b(BuiltIn(Cons)),
                            b(Apply(
                                b(BuiltIn(MultiDraw)),
                                b(Apply(
                                    b(BuiltIn(Head)),
                                    b(Apply(
                                        b(BuiltIn(Tail)),
                                        b(Apply(b(BuiltIn(Tail)), arg1.clone())),
                                    )),
                                )),
                            )),
                        )),
                        b(BuiltIn(Nil)),
                    )),
                )),
            )),
            b(Apply(
                b(Apply(
                    b(Apply(b(BuiltIn(Interact)), arg0)),
                    b(Apply(
                        b(BuiltIn(I)), // TODO: modem
                        b(Apply(
                            b(BuiltIn(Head)),
                            b(Apply(b(BuiltIn(Tail)), arg1.clone())),
                        )),
                    )),
                )),
                b(Apply(
                    b(BuiltIn(Send)),
                    b(Apply(
                        b(BuiltIn(Head)),
                        b(Apply(b(BuiltIn(Tail)), b(Apply(b(BuiltIn(Tail)), arg1)))),
                    )),
                )),
            )),
        )
    }

    fn eval_multidraw(&self, val: Value) -> Value {
        // println!("multidraw lazy: {:?}", val);
        let val = self.eval_value(val, false);
        // println!("multidraw eager: {:?}", val);
        if Value::BuiltIn(BuiltIn::Nil) == val {
            return val;
        }
        if let Value::Apply(f1, tail) = val {
            if let Value::Apply(f0, head) = *f1 {
                if let Value::BuiltIn(BuiltIn::Cons) = *f0 {
                    return Value::Apply(
                        Box::new(Value::Apply(
                            Box::new(Value::BuiltIn(BuiltIn::Cons)),
                            Box::new(Value::Picture(self.eval_draw(*head))),
                        )),
                        Box::new(self.eval_multidraw(*tail)),
                    );
                }
            }
        }
        panic!("Invalid multidraw argument")
    }

    fn eval_draw(&self, val: Value) -> Picture {
        // println!("eval_draw: {:?}", val);
        let mut picture = Picture::new();
        let mut list = self.eval_nested_list(val);
        // println!("eval_draw: {:?}", list);
        loop {
            // we expect a list of pairs here
            match list {
                NestedList::Nil => break,
                NestedList::Cons(head, tail) => {
                    match *head {
                        NestedList::Cons(x, y) => {
                            if let NestedList::Number(x) = *x {
                                if let NestedList::Number(y) = *y {
                                    picture.add(x as u32, y as u32);
                                } else {
                                    panic!("Invalid list")
                                }
                            } else {
                                panic!("Invalid list")
                            }
                        }
                        _ => panic!("Invalid list"),
                    }
                    list = *tail;
                }
                _ => panic!("Invalid list"),
            }
        }
        picture
    }

    fn eval_nested_list(&self, val: Value) -> NestedList {
        match val {
            Value::Apply(f1, tail) => {
                if let Value::Apply(f0, head) = *f1 {
                    if let Value::BuiltIn(BuiltIn::Cons) = *f0 {
                        NestedList::Cons(
                            Box::new(self.eval_nested_list(*head)),
                            Box::new(self.eval_nested_list(*tail)),
                        )
                    } else {
                        panic!("Invalid list format")
                    }
                } else {
                    panic!("Invalid list format")
                }
            }
            Value::BuiltIn(BuiltIn::Nil) => NestedList::Nil,
            Value::Number(n) => NestedList::Number(n),
            _ => panic!("Invalid value in eval_list: {:?}", val),
        }
    }

    pub fn interpret(&mut self, stmt: Stmt) {
        // println!("Compiling {:?}", stmt.var);
        // println!("Raw: {:?}", stmt.code);
        let v = self.compile(stmt.code);
        // println!("Compiled: {:?}", v);
        self.vars.insert(stmt.var, v);
    }

    fn compile(&self, code: Vec<Token>) -> Value {
        let mut stack: Vec<Value> = vec![];
        for token in code.into_iter().rev() {
            match token {
                Token::Var(v) => stack.push(Value::Var(v)),

                Token::Number(n) => stack.push(Value::Number(n)),
                Token::True => stack.push(Value::BuiltIn(BuiltIn::True)),
                Token::False => stack.push(Value::BuiltIn(BuiltIn::False)),
                Token::Nil => stack.push(Value::BuiltIn(BuiltIn::Nil)),

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
                Token::MultiDraw => stack.push(Value::BuiltIn(BuiltIn::MultiDraw)),
                Token::If0 => stack.push(Value::BuiltIn(BuiltIn::If0)),
                Token::Interact => stack.push(Value::BuiltIn(BuiltIn::Interact)),

                Token::Ap => {
                    let x = stack.pop().unwrap();
                    let v = stack.pop().unwrap();
                    stack.push(Value::Apply(Box::new(x), Box::new(v)));
                }
            }
        }
        assert!(stack.len() == 1);
        stack[0].clone()
    }
}
