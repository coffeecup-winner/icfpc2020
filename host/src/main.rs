use std::{env, fs, io};

// TODO: don't create new strings if have time/poor perf
#[derive(Debug)]
enum Token {
    Number(i64), // #1-4 increase to u64 if needed
    Inc,         // #5
    Dec,         // #6
    Add,         // #7
    Var(u32),    // #8 - ???
    Mul,         // #9
    Div,         // #10
    Eq,          // #11
    Lt,          // #12
    Mod,         // #13 - ???
    Dem,         // #14 - ???
    Send,        // #15 - ???
    Neg,         // #16
    Ap,          // #17
    S,           // #18
    C,           // #19
    B,           // #20
    True,        // #21
    False,       // #22
    Pwr2,        // #23 - ???
    I,           // #24
    Cons,        // #25
    Head,        // #26
    Tail,        // #27
    Nil,         // #28
    IsNil,       // #29
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
    String(String),
}

#[derive(Debug)]
enum Var {
    Named(String),
    Temp(u32),
}

#[derive(Debug)]
struct Stmt {
    var: Var,
    code: Vec<Token>,
}

fn parse(text: &str) -> Vec<Token> {
    let parts: Vec<&str> = text.split(" ").collect();
    parts
        .iter()
        .map(|s| match s {
            // Inc, // #5
            // Dec, // #6
            &"add" => Token::Add,
            &"mul" => Token::Mul,
            &"div" => Token::Div,
            &"eq" => Token::Eq,
            &"lt" => Token::Lt,
            // Mod, // #13 - ???
            // Dem, // #14 - ???
            // Send, // #15 - ???
            &"neg" => Token::Neg,
            &"ap" => Token::Ap,
            &"s" => Token::S,
            &"c" => Token::C,
            &"b" => Token::B,
            &"t" => Token::True,
            // False, // #22
            // Pwr2, // #23 - ???
            &"i" => Token::I,
            &"cons" => Token::Cons,
            &"car" => Token::Head,
            &"cdr" => Token::Tail,
            &"nil" => Token::Nil,
            &"isnil" => Token::IsNil,
            // // #30 - ???
            // // #31 - ???
            // Draw, // #32
            // Checkerboard, // #33
            // MultiDraw, // #34
            // ModList, // #35 - ???
            // Send2, // #36 - ???
            // If0, // #37
            // Interact, // #38-39 - ???
            // StatelessDraw, // #40 - ???
            // StatefulDraw, // #41 - ???
            // Galaxy, // #42
            s if s.starts_with(":") => {
                Token::Var(s.strip_prefix(":").unwrap().parse::<u32>().unwrap())
            }
            s if s.chars().all(|c| c.is_ascii_digit())
                || s.starts_with("-") && s.chars().skip(1).all(|c| c.is_ascii_digit()) =>
            {
                Token::Number(s.parse::<i64>().unwrap())
            }
            _ => panic!(format!("{}", s)),
        })
        .collect()
}

fn parse_line(text: &str) -> Stmt {
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

fn main() -> io::Result<()> {
    let file = fs::read_to_string(env::args().nth(1).unwrap())?;
    for line in file.lines() {
        let stmt = parse_line(line);
        // println!("{:?}", stmt);
        // return Ok(());
    }
    Ok(())
}
