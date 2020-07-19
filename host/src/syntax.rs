#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Token {
    Number(i64), // #1-4 increase to u64 if needed
    Inc,         // #5
    Dec,         // #6
    Add,         // #7
    Var(Var),    // #8
    Mul,         // #9
    Div,         // #10
    Eq,          // #11
    Lt,          // #12
    Neg,         // #16
    Ap,          // #17
    S,           // #18
    C,           // #19
    B,           // #20
    True,        // #21
    False,       // #22
    Pwr2,        // #23
    I,           // #24
    Cons,        // #25
    Head,        // #26
    Tail,        // #27
    Nil,         // #28
    IsNil,       // #29
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Var {
    Named(String),
    Temp(u32),
}

#[derive(Debug)]
pub struct Stmt {
    pub var: Var,
    pub code: Vec<Token>,
}

fn parse(text: &str) -> Vec<Token> {
    let parts: Vec<&str> = text.split(' ').collect();
    let mut new_item = false;
    let mut result = vec![];
    for s in parts {
        if s == ")" {
            new_item = false;
            result.push(Token::Nil);
        } else {
            if new_item {
                result.push(Token::Ap);
                result.push(Token::Ap);
                result.push(Token::Cons);
                new_item = false;
            }
            match s {
                "inc" => result.push(Token::Inc),
                "dec" => result.push(Token::Dec),
                "add" => result.push(Token::Add),
                "mul" => result.push(Token::Mul),
                "div" => result.push(Token::Div),
                "eq" => result.push(Token::Eq),
                "lt" => result.push(Token::Lt),
                "neg" => result.push(Token::Neg),
                "ap" => result.push(Token::Ap),
                "s" => result.push(Token::S),
                "c" => result.push(Token::C),
                "b" => result.push(Token::B),
                "t" => result.push(Token::True),
                "f" => result.push(Token::False),
                "pwr2" => result.push(Token::Pwr2),
                "i" => result.push(Token::I),
                "cons" | "vec" => result.push(Token::Cons),
                "car" => result.push(Token::Head),
                "cdr" => result.push(Token::Tail),
                "nil" => result.push(Token::Nil),
                "isnil" => result.push(Token::IsNil),
                "(" => new_item = true,
                "," => new_item = true,
                s if s.starts_with(':') => result.push(Token::Var(Var::Temp(
                    s.strip_prefix(':').unwrap().parse::<u32>().unwrap(),
                ))),
                s if s.chars().all(|c| c.is_ascii_digit())
                    || s.starts_with('-') && s.chars().skip(1).all(|c| c.is_ascii_digit()) =>
                {
                    result.push(Token::Number(s.parse::<i64>().unwrap()))
                }
                _ => panic!("{}", s),
            }
        }
    }
    result
}

pub fn parse_line(text: &str) -> Stmt {
    let parts: Vec<&str> = text.split(" = ").collect();
    let var = match parts[0].strip_prefix(":") {
        Some(s) => Var::Temp(s.parse().unwrap()),
        None => Var::Named(parts[0].to_string()),
    };
    Stmt {
        var,
        code: parse(parts[1]),
    }
}

pub fn parse_test(text: &str) -> (Stmt, Stmt) {
    let parts: Vec<&str> = text.split("==").collect();
    (
        Stmt {
            var: Var::Named("expr".to_string()),
            code: parse(parts[0].trim()),
        },
        Stmt {
            var: Var::Named("expected".to_string()),
            code: parse(parts[1].trim()),
        },
    )
}

pub fn parse_picture(text: &str) -> Stmt {
    let parts: Vec<&str> = text.split("==").collect();
    Stmt {
        var: Var::Named("picture".to_string()),
        code: parse(parts[0].trim()),
    }
}
