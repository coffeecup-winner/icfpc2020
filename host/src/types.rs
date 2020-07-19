use crate::eval::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Picture {
    pub offset_x: i64,
    pub offset_y: i64,
    pub width: u32,
    pub height: u32,
    pub points: Vec<Point>,
}

#[derive(Debug, Default)]
pub struct PictureBuilder {
    points: Vec<(i64, i64)>,
}

impl PictureBuilder {
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
        let mut points = vec![];
        loop {
            // we expect a list of pairs here
            match list {
                NestedList::Nil => break,
                NestedList::Cons(head, tail) => {
                    match *head {
                        NestedList::Cons(x, y) => {
                            if let NestedList::Number(x) = *x {
                                if let NestedList::Number(y) = *y {
                                    points.push((x, y));
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
        Self::from_points(points)
    }

    fn from_points(points: Vec<(i64, i64)>) -> Picture {
        if points.is_empty() {
            Picture::default()
        } else {
            let (mut min_x, mut min_y) = points[0];
            let (mut max_x, mut max_y) = points[0];
            for &(x, y) in points.iter() {
                if min_x > x {
                    min_x = x;
                }
                if max_x < x {
                    max_x = x;
                }
                if min_y > y {
                    min_y = y;
                }
                if max_y < y {
                    max_y = y;
                }
            }
            let offset_x = -min_x;
            let offset_y = -min_y;
            let mut pic_points = vec![];
            for (x, y) in points {
                pic_points.push(Point {
                    x: (x + offset_x) as u32,
                    y: (y + offset_y) as u32,
                });
            }
            Picture {
                offset_x,
                offset_y,
                width: (max_x - min_x) as u32,
                height: (max_y - min_y) as u32,
                points: pic_points,
            }
        }
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
