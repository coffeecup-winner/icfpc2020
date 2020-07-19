use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::modem::{self, NestedList};
use crate::send;
use crate::syntax::{Stmt, Token, Var};

#[derive(Debug, Default)]
pub struct State {
    vars: HashMap<Var, Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value_ {
    Var(Var),
    Number(i64),
    Signal(Vec<bool>), // used with modulate / demodulate
    Picture(Picture),
    BuiltIn(BuiltIn),
    Apply(Value, Value),
}

fn var(v: Var) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::Var(v),
        computed: false,
    }))
}

fn number(n: i64) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::Number(n),
        computed: true,
    }))
}

fn signal(s: Vec<bool>) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::Signal(s),
        computed: true,
    }))
}

fn picture(p: Picture) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::Picture(p),
        computed: true,
    }))
}

pub fn b(b: BuiltIn) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::BuiltIn(b),
        computed: true,
    }))
}

fn ap(f: Value, arg: Value) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::Apply(f, arg),
        computed: false,
    }))
}

#[derive(Debug, PartialEq, Clone)]
pub struct V {
    val: Value_,
    computed: bool,
}

pub type Value = Rc<RefCell<V>>;

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

    // This will put a single picture into a vector as well
    pub fn eval_picture_list(&self, val: Value) -> Vec<Picture> {
        if let Value_::Picture(p) = &val.borrow().val {
            return vec![p.clone()];
        } else {
            let mut curr = val.clone();
            let mut result = vec![];
            loop {
                if let Value_::Apply(f0, tail) = &curr.clone().borrow().val {
                    if let Value_::Apply(f1, head) = &f0.borrow().val {
                        if let Value_::BuiltIn(BuiltIn::Cons) = &f1.borrow().val {
                            if let Value_::Picture(p) = &head.borrow().val {
                                result.push(p.clone());
                                curr = tail.clone();
                                continue;
                            }
                        }
                    }
                } else if Value_::BuiltIn(BuiltIn::Nil) == curr.borrow().val {
                    break;
                }
                panic!("Not a picture: {:?}", val);
            }
            result
        }
    }

    pub fn eval_v(&self, var: &Var) -> Value {
        let v = self.vars.get(var).unwrap();
        self.eval(v.clone())
    }

    pub fn eval(&self, val: Value) -> Value {
        if val.borrow().computed {
            return val;
        }
        let mut curr = val.clone();
        loop {
            let new = self.eval_core(curr.clone());
            if new == curr {
                let value = new.borrow().val.clone();
                val.borrow_mut().val = value;
                val.borrow_mut().computed = true;
                break val;
            }
            curr = new.clone();
        }
    }

    fn eval_core(&self, val: Value) -> Value {
        // println!("eval_value: {:?}", val);
        let value = val.borrow().val.clone();
        match &value {
            Value_::Var(v) => self.vars.get(&v).unwrap().clone(),
            Value_::Number(_) => val,
            Value_::Signal(_) => val,
            Value_::Picture(_) => val,
            Value_::BuiltIn(_) => val,
            Value_::Apply(f0, arg0) => {
                match &self.eval(f0.clone()).borrow().val {
                    Value_::BuiltIn(BuiltIn::Send) => {
                        let arg0 = self.eval_nested_list(arg0.clone());
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
                        panic!("demodulated to: {:?}", demodulated);
                    }
                    Value_::BuiltIn(BuiltIn::Inc) => {
                        if let Value_::Number(n) = self.eval(arg0.clone()).borrow().val {
                            number(n + 1)
                        } else {
                            panic!("Invalid argument for `inc`");
                        }
                    }
                    Value_::BuiltIn(BuiltIn::Dec) => {
                        if let Value_::Number(n) = self.eval(arg0.clone()).borrow().val {
                            number(n - 1)
                        } else {
                            panic!("Invalid argument for `dec`");
                        }
                    }
                    Value_::BuiltIn(BuiltIn::Mod) => signal(modem::mod_list(
                        &self.eval_nested_list(self.eval(arg0.clone())),
                    )),
                    Value_::BuiltIn(BuiltIn::Dem) => {
                        if let Value_::Signal(s) = &self.eval(arg0.clone()).borrow().val {
                            let list = modem::dem_list(s);
                            panic!("TODO: demodulate {:?}", list);
                        } else {
                            panic!("Invalid argument for `dem`");
                        }
                    }
                    Value_::BuiltIn(BuiltIn::Neg) => {
                        if let Value_::Number(n) = self.eval(arg0.clone()).borrow().val {
                            number(-n)
                        } else {
                            panic!("Invalid argument for `neg`");
                        }
                    }
                    Value_::BuiltIn(BuiltIn::Pwr2) => {
                        if let Value_::Number(n) = self.eval(arg0.clone()).borrow().val {
                            number((2 as i64).pow(n as u32))
                        } else {
                            panic!("Invalid argument for `pwr2`");
                        }
                    }
                    Value_::BuiltIn(BuiltIn::I) => arg0.clone(),
                    Value_::BuiltIn(BuiltIn::Head) => ap(arg0.clone(), b(BuiltIn::True)),
                    Value_::BuiltIn(BuiltIn::Tail) => ap(arg0.clone(), b(BuiltIn::False)),
                    Value_::BuiltIn(BuiltIn::Nil) => b(BuiltIn::True),
                    Value_::BuiltIn(BuiltIn::IsNil) => ap(
                        arg0.clone(),
                        ap(b(BuiltIn::True), ap(b(BuiltIn::True), b(BuiltIn::False))),
                    ),
                    Value_::BuiltIn(BuiltIn::Draw) => picture(self.eval_draw(arg0.clone())),
                    Value_::BuiltIn(BuiltIn::MultiDraw) => self.eval_multidraw(arg0.clone()),

                    // ===== Arity 2 =====
                    Value_::Apply(f1, arg1) => {
                        match &self.eval(f1.clone()).borrow().val {
                            Value_::BuiltIn(BuiltIn::Add) => {
                                if let Value_::Number(y) = self.eval(arg0.clone()).borrow().val {
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val {
                                        number(x + y)
                                    } else {
                                        panic!("Invalid argument for `add`");
                                    }
                                } else {
                                    panic!("Invalid argument for `add`");
                                }
                            }
                            Value_::BuiltIn(BuiltIn::Mul) => {
                                if let Value_::Number(y) = self.eval(arg0.clone()).borrow().val {
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val {
                                        number(x * y)
                                    } else {
                                        panic!("Invalid argument for `mul`");
                                    }
                                } else {
                                    panic!("Invalid argument for `mul`");
                                }
                            }
                            Value_::BuiltIn(BuiltIn::Div) => {
                                if let Value_::Number(y) = self.eval(arg0.clone()).borrow().val {
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val {
                                        number(x / y)
                                    } else {
                                        panic!("Invalid argument for `div`");
                                    }
                                } else {
                                    panic!("Invalid argument for `div`");
                                }
                            }
                            Value_::BuiltIn(BuiltIn::Eq) => {
                                if let Value_::Number(y) = self.eval(arg0.clone()).borrow().val {
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val {
                                        if x == y {
                                            b(BuiltIn::True)
                                        } else {
                                            b(BuiltIn::False)
                                        }
                                    } else {
                                        panic!("Invalid argument for `eq`");
                                    }
                                } else {
                                    panic!("Invalid argument for `eq`");
                                }
                            }
                            Value_::BuiltIn(BuiltIn::Lt) => {
                                if let Value_::Number(y) = self.eval(arg0.clone()).borrow().val {
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val {
                                        if x < y {
                                            b(BuiltIn::True)
                                        } else {
                                            b(BuiltIn::False)
                                        }
                                    } else {
                                        panic!("Invalid argument for `lt`");
                                    }
                                } else {
                                    panic!("Invalid argument for `lt`");
                                }
                            }
                            Value_::BuiltIn(BuiltIn::True) => arg1.clone(),
                            Value_::BuiltIn(BuiltIn::False) => arg0.clone(),
                            Value_::BuiltIn(BuiltIn::F38) => {
                                Self::construct_f38_builtin(arg1.clone(), arg0.clone())
                            }

                            // ===== Arity 3 =====
                            Value_::Apply(f2, arg2) => match &self.eval(f2.clone()).borrow().val {
                                Value_::BuiltIn(BuiltIn::S) => {
                                    ap(ap(arg2.clone(), arg0.clone()), ap(arg1.clone(), arg0.clone()))
                                }
                                Value_::BuiltIn(BuiltIn::C) => ap(ap(arg2.clone(), arg0.clone()), arg1.clone()),
                                Value_::BuiltIn(BuiltIn::B) => ap(arg2.clone(), ap(arg1.clone(), arg0.clone())),
                                Value_::BuiltIn(BuiltIn::Cons) => ap(ap(arg0.clone(), arg2.clone()), arg1.clone()),
                                Value_::BuiltIn(BuiltIn::If0) => {
                                    if let Value_::Number(0) = self.eval(arg2.clone()).borrow().val {
                                        arg1.clone()
                                    } else {
                                        arg0.clone()
                                    }
                                }
                                Value_::BuiltIn(BuiltIn::Interact) => {
                                    ap(ap(b(BuiltIn::F38), arg2.clone()), ap(ap(arg2.clone(), arg1.clone()), arg0.clone()))
                                }
                                _ => val,
                            },
                            _ => val,
                        }
                    }
                    _ => val,
                }
            }
        }
    }

    fn construct_f38_builtin(arg0: Value, arg1: Value) -> Value {
        use self::BuiltIn::*;
        ap(
            ap(
                ap(b(If0), ap(b(Head), arg1.clone())),
                ap(
                    ap(
                        b(Cons),
                        ap(
                            b(I), // TODO: modem
                            ap(b(Head), ap(b(Tail), arg1.clone())),
                        ),
                    ),
                    ap(
                        ap(
                            b(Cons),
                            ap(b(MultiDraw), ap(b(Head), ap(b(Tail), ap(b(Tail), arg1.clone())))),
                        ),
                        b(Nil),
                    ),
                ),
            ),
            ap(
                ap(
                    ap(b(Interact), arg0),
                    ap(
                        b(I), // TODO: modem
                        ap(b(Head), ap(b(Tail), arg1.clone())),
                    ),
                ),
                ap(b(Send), ap(b(Head), ap(b(Tail), ap(b(Tail), arg1)))),
            ),
        )
    }

    fn eval_multidraw(&self, val: Value) -> Value {
        // println!("multidraw lazy: {:?}", val);
        let val = self.eval(val);
        // println!("multidraw eager: {:?}", val);
        if Value_::BuiltIn(BuiltIn::Nil) == val.borrow().val {
            return val;
        }
        if let Value_::Apply(f1, tail) = &val.borrow().val {
            if let Value_::Apply(f0, head) = &f1.borrow().val {
                if let Value_::BuiltIn(BuiltIn::Cons) = &f0.borrow().val {
                    return ap(
                        ap(b(BuiltIn::Cons), picture(self.eval_draw(head.clone()))),
                        self.eval_multidraw(tail.clone()),
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
        match &val.borrow().val {
            Value_::Apply(f1, tail) => {
                if let Value_::Apply(f0, head) = &f1.borrow().val {
                    if let Value_::BuiltIn(BuiltIn::Cons) = &f0.borrow().val {
                        NestedList::Cons(
                            Box::new(self.eval_nested_list(head.clone())),
                            Box::new(self.eval_nested_list(tail.clone())),
                        )
                    } else {
                        panic!("Invalid list format")
                    }
                } else {
                    panic!("Invalid list format")
                }
            }
            Value_::BuiltIn(BuiltIn::Nil) => NestedList::Nil,
            Value_::Number(n) => NestedList::Number(*n),
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
                Token::Var(v) => stack.push(var(v)),

                Token::Number(n) => stack.push(number(n)),
                Token::True => stack.push(b(BuiltIn::True)),
                Token::False => stack.push(b(BuiltIn::False)),
                Token::Nil => stack.push(b(BuiltIn::Nil)),

                Token::Inc => stack.push(b(BuiltIn::Inc)),
                Token::Dec => stack.push(b(BuiltIn::Dec)),
                Token::Add => stack.push(b(BuiltIn::Add)),
                Token::Mul => stack.push(b(BuiltIn::Mul)),
                Token::Div => stack.push(b(BuiltIn::Div)),
                Token::Eq => stack.push(b(BuiltIn::Eq)),
                Token::Lt => stack.push(b(BuiltIn::Lt)),
                Token::Mod => stack.push(b(BuiltIn::Mod)),
                Token::Dem => stack.push(b(BuiltIn::Dem)),
                Token::Send => stack.push(b(BuiltIn::Send)),
                Token::Neg => stack.push(b(BuiltIn::Neg)),
                Token::S => stack.push(b(BuiltIn::S)),
                Token::C => stack.push(b(BuiltIn::C)),
                Token::B => stack.push(b(BuiltIn::B)),
                Token::Pwr2 => stack.push(b(BuiltIn::Pwr2)),
                Token::I => stack.push(b(BuiltIn::I)),
                Token::Cons => stack.push(b(BuiltIn::Cons)),
                Token::Head => stack.push(b(BuiltIn::Head)),
                Token::Tail => stack.push(b(BuiltIn::Tail)),
                Token::IsNil => stack.push(b(BuiltIn::IsNil)),
                Token::Draw => stack.push(b(BuiltIn::Draw)),
                Token::MultiDraw => stack.push(b(BuiltIn::MultiDraw)),
                Token::If0 => stack.push(b(BuiltIn::If0)),
                Token::Interact => stack.push(b(BuiltIn::Interact)),

                Token::Ap => {
                    let x = stack.pop().unwrap();
                    let v = stack.pop().unwrap();
                    stack.push(ap(x, v));
                }
            }
        }
        assert!(stack.len() == 1);
        stack[0].clone()
    }
}
