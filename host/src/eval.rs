use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::syntax::{Stmt, Token, Var};

#[derive(Debug, Default)]
pub struct State {
    vars: HashMap<Var, Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value_ {
    Var(Var),
    Number(i64),
    BuiltIn(BuiltIn),
    Apply(Value, Value),
}

pub fn var(v: Var) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::Var(v),
        computed: false,
    }))
}

pub fn number(n: i64) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::Number(n),
        computed: true,
    }))
}

pub fn b(b: BuiltIn) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::BuiltIn(b),
        computed: true,
    }))
}

pub fn ap(f: Value, arg: Value) -> Value {
    Rc::new(RefCell::new(V {
        val: Value_::Apply(f, arg),
        computed: false,
    }))
}

#[derive(Debug, PartialEq, Clone)]
pub struct V {
    pub val: Value_,
    computed: bool,
}

pub type Value = Rc<RefCell<V>>;

// Built-in functions except `ap`
#[derive(Debug, PartialEq, Clone)]
pub enum BuiltIn {
    Inc,   // #5
    Dec,   // #6
    Add,   // #7
    Mul,   // #9
    Div,   // #10
    Eq,    // #11
    Lt,    // #12
    Neg,   // #16
    S,     // #18
    C,     // #19
    B,     // #20
    True,  // #21
    False, // #22
    Pwr2,  // #23
    I,     // #24
    Cons,  // #25
    Head,  // #26
    Tail,  // #27
    Nil,   // #28
    IsNil, // #29
}

impl State {
    pub fn new() -> Self {
        State::default()
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
        if val.borrow().computed {
            return val;
        }
        let value = val.borrow().val.clone();
        match &value {
            Value_::Var(v) => self.vars.get(&v).unwrap().clone(),
            Value_::Number(_) => val,
            Value_::BuiltIn(_) => val,
            Value_::Apply(f0, arg0) => {
                match &self.eval(f0.clone()).borrow().val {
                    // Value_::BuiltIn(BuiltIn::Send) => {
                    //     let arg0 = self.eval_nested_list(arg0.clone());
                    //     let signal = modem::mod_list(&arg0);
                    //     let signal_str = signal
                    //         .into_iter()
                    //         .map(|x| if x { '1' } else { '0' })
                    //         .collect();
                    //     let endpoint =
                    //         String::from("https://icfpc2020-api.testkontur.ru/aliens/send");
                    //     let token = std::env::var("ICFPC_TEAM_TOKEN").ok();
                    //     let response = match send::request(&endpoint, token, &signal_str) {
                    //         Ok(val) => val,
                    //         Err(err) => panic!("request failed: {:?}", err),
                    //     };

                    //     let demodulated =
                    //         match modem::demodulate(&mut response.chars().map(|x| x != '0')) {
                    //             Ok(val) => val,
                    //             Err(err) => panic!("demodulation failed: {:?}", err),
                    //         };
                    //     panic!("demodulated to: {:?}", demodulated);
                    // }
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

                    // ===== Arity 2 =====
                    Value_::Apply(f1, arg1) => {
                        match &self.eval(f1.clone()).borrow().val {
                            Value_::BuiltIn(BuiltIn::Add) => {
                                if let Value_::Number(y) = self.eval(arg0.clone()).borrow().val {
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val
                                    {
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
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val
                                    {
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
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val
                                    {
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
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val
                                    {
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
                                    if let Value_::Number(x) = self.eval(arg1.clone()).borrow().val
                                    {
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
                            Value_::BuiltIn(BuiltIn::Cons) => {
                                let cons = ap(
                                    ap(b(BuiltIn::Cons), self.eval(arg1.clone())),
                                    self.eval(arg0.clone()),
                                );
                                cons.borrow_mut().computed = true;
                                cons
                            }

                            // ===== Arity 3 =====
                            Value_::Apply(f2, arg2) => match &self.eval(f2.clone()).borrow().val {
                                Value_::BuiltIn(BuiltIn::S) => ap(
                                    ap(arg2.clone(), arg0.clone()),
                                    ap(arg1.clone(), arg0.clone()),
                                ),
                                Value_::BuiltIn(BuiltIn::C) => {
                                    ap(ap(arg2.clone(), arg0.clone()), arg1.clone())
                                }
                                Value_::BuiltIn(BuiltIn::B) => {
                                    ap(arg2.clone(), ap(arg1.clone(), arg0.clone()))
                                }
                                Value_::BuiltIn(BuiltIn::Cons) => {
                                    ap(ap(arg0.clone(), arg2.clone()), arg1.clone())
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
