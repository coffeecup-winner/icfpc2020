use crate::eval::*;

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

    pub fn from_nested_list(mut list: NestedList) -> Vec<Picture> {
        let mut result = vec![];
        loop {
            match list {
                NestedList::Nil => break,
                NestedList::Cons(head, tail) => {
                    result.push(Self::from_nested_list_one(*head));
                    list = *tail;
                }
                _ => panic!("Invalid list"),
            }
        }
        result
    }

    fn from_nested_list_one(mut list: NestedList) -> Picture {
        let mut picture = Picture::new();
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

#[derive(Debug, PartialEq, Clone)]
pub enum NestedList {
    Nil,
    Cons(Box<NestedList>, Box<NestedList>),
    Number(i64),
}

impl NestedList {
    pub fn from_value(val: Value) -> NestedList {
        // println!("{:?}", val);
        match &val.borrow().val {
            Value_::Apply(f1, tail) => {
                if let Value_::Apply(f0, head) = &f1.borrow().val {
                    if let Value_::BuiltIn(BuiltIn::Cons) = &f0.borrow().val {
                        NestedList::Cons(
                            Box::new(Self::from_value(head.clone())),
                            Box::new(Self::from_value(tail.clone())),
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

    pub fn into_value(self) -> Value {
        match self {
            NestedList::Nil => b(BuiltIn::Nil),
            NestedList::Cons(head, tail) => {
                ap(ap(b(BuiltIn::Cons), head.into_value()), tail.into_value())
            }
            NestedList::Number(n) => number(n),
        }
    }
}
