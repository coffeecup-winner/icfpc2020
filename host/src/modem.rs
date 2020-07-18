#[derive(Debug, PartialEq, Clone)]
pub enum NestedList {
    Nil,
    Cons(Box<NestedList>, Box<NestedList>),
    Number(i64),
}

pub fn mod_num(number: i64) -> Vec<bool> {
    let mut res: Vec<bool> = vec![];
    modulate(number, &mut res);
    res
}

pub fn mod_list(list: &NestedList) -> Vec<bool> {
    panic!()
}

pub fn dem_num(data: &Vec<bool>) -> i64 {
    panic!()
}

pub fn dem_list(data: &Vec<bool>) -> NestedList {
    panic!()
}

fn modulate(signed_num: i64, res: &mut Vec<bool>) -> () {
    if signed_num >= 0 {
        res.push(false);
        res.push(true);
    } else {
        res.push(true);
        res.push(false);
    }

    let num = {
        if signed_num >= 0 {
            signed_num as u64
        } else {
            (-signed_num) as u64
        }
    };

    // count the number of nibbles required to represent the number
    let unused_space = num.leading_zeros();
    let used_space = 64 - unused_space;
    let needed_nibbles = (used_space + 3) / 4;

    for _ in 0..needed_nibbles {
        res.push(true);
    }
    res.push(false);

    let encoded_space = needed_nibbles * 4;
    for i in 0..encoded_space {
        // MSB first
        let mask = 1u64 << encoded_space - 1 - i;
        res.push(num & mask != 0);
    }
}

fn demodulate_value(negative: bool, iter: &mut Iterator<Item = bool>) -> Result<i64, ()> {
    let used_nibbles = 'outer: loop {
        for i in 0u64.. {
            if !match iter.next() {
                Some(val) => val,
                None => return Err(()),
            } {
                break 'outer i;
            }
        }
    };

    let mut res = 0u64;
    for i in 0..(used_nibbles * 4) {
        if match iter.next() {
            Some(val) => val,
            None => return Err(()),
        } {
            res |= 1u64 << i;
        }
    }

    Ok(if negative { -(res as i64) } else { res as i64 })
}

#[cfg(test)]
mod tests {
    use super::*;
    use NestedList::*;

    fn v(s: &str) -> Vec<bool> {
        s.chars()
            .map(|c| if c == '1' { true } else { false })
            .collect()
    }

    fn cons(a: NestedList, b: NestedList) -> NestedList {
        Cons(Box::new(a), Box::new(b))
    }

    #[test]
    fn test_mod_num() {
        assert_eq!(mod_num(0), v("010"));
        assert_eq!(mod_num(1), v("01100001"));
        assert_eq!(mod_num(-1), v("10100001"));
        assert_eq!(mod_num(2), v("01100010"));
        assert_eq!(mod_num(-2), v("10100010"));
        assert_eq!(mod_num(16), v("0111000010000"));
        assert_eq!(mod_num(-16), v("1011000010000"));
        assert_eq!(mod_num(255), v("0111011111111"));
        assert_eq!(mod_num(-255), v("1011011111111"));
        assert_eq!(mod_num(256), v("011110000100000000"));
        assert_eq!(mod_num(-256), v("101110000100000000"));
    }

    #[test]
    fn test_mod_list() {
        assert_eq!(mod_list(&Nil), v("00"));
        assert_eq!(mod_list(&cons(Nil, Nil)), v("110000"));
        assert_eq!(mod_list(&cons(Number(0), Nil)), v("1101000"));
        let one_two_nil = v("110110000101100010");
        assert_eq!(mod_list(&cons(Number(1), Number(2))), one_two_nil);
        assert_eq!(mod_list(&cons(Number(1), cons(Number(2), Nil))), one_two_nil);
        let second_item = cons(Number(2), cons(Number(3), Nil));
        let woosh = cons(Number(1), cons(second_item, cons(Number(4), Nil)));;
        assert_eq!(mod_list(&woosh), v("110110000111110110001011011000110011011001000"));
    }
}
