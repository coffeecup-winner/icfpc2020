use crate::types::NestedList;

pub fn mod_list(list: &NestedList) -> Vec<bool> {
    let mut res: Vec<bool> = vec![];
    modulate(list, &mut res);
    res
}

pub fn dem_list(data: &[bool]) -> NestedList {
    demodulate(&mut data.iter().copied()).unwrap()
}

fn modulate_value(signed_num: i64, res: &mut Vec<bool>) {
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
        let mask = 1u64 << (encoded_space - 1 - i);
        res.push(num & mask != 0);
    }
}

fn modulate(val: &NestedList, res: &mut Vec<bool>) {
    use NestedList::*;

    match val {
        Number(number) => modulate_value(*number, res),
        Nil => {
            res.push(false);
            res.push(false);
        }
        Cons(a, b) => {
            res.push(true);
            res.push(true);
            modulate(a, res);
            modulate(b, res);
        }
    }
}

fn iter_next(iter: &mut dyn Iterator<Item = bool>) -> Result<bool, ()> {
    iter.next().ok_or(())
}

fn demodulate_value(negative: bool, iter: &mut dyn Iterator<Item = bool>) -> Result<i64, ()> {
    let used_nibbles = 'outer: loop {
        for i in 0u64.. {
            if !iter_next(iter)? {
                break 'outer i;
            }
        }
    };

    let mut res = 0u64;
    let bit_size = (used_nibbles * 4);
    for i in 0..bit_size {
        if iter_next(iter)? {
            res |= 1u64 << (bit_size - i - 1);
        }
    }

    Ok(if negative { -(res as i64) } else { res as i64 })
}

fn demodulate(iter: &mut dyn Iterator<Item = bool>) -> Result<NestedList, ()> {
    use NestedList::*;

    let a = iter_next(iter)?;
    let b = iter_next(iter)?;

    match (a, b) {
        (false, false) => Ok(Nil),
        (true, true) => {
            let left = demodulate(iter)?;
            let right = demodulate(iter)?;
            Ok(Cons(Box::new(left), Box::new(right)))
        }
        (neg, _) => Ok(Number(demodulate_value(neg, iter)?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use NestedList::*;

    fn n(num: i64) -> NestedList {
        NestedList::Number(num)
    }

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
        assert_eq!(mod_list(&n(0)), v("010"));
        assert_eq!(mod_list(&n(1)), v("01100001"));
        assert_eq!(mod_list(&n(-1)), v("10100001"));
        assert_eq!(mod_list(&n(2)), v("01100010"));
        assert_eq!(mod_list(&n(-2)), v("10100010"));
        assert_eq!(mod_list(&n(16)), v("0111000010000"));
        assert_eq!(mod_list(&n(-16)), v("1011000010000"));
        assert_eq!(mod_list(&n(255)), v("0111011111111"));
        assert_eq!(mod_list(&n(-255)), v("1011011111111"));
        assert_eq!(mod_list(&n(256)), v("011110000100000000"));
        assert_eq!(mod_list(&n(-256)), v("101110000100000000"));
    }

    #[test]
    fn test_mod_list() {
        assert_eq!(mod_list(&Nil), v("00"));
        assert_eq!(mod_list(&cons(Nil, Nil)), v("110000"));
        assert_eq!(mod_list(&cons(Number(0), Nil)), v("1101000"));
        assert_eq!(
            mod_list(&cons(Number(1), Number(2))),
            v("110110000101100010")
        );
        assert_eq!(
            mod_list(&cons(Number(1), cons(Number(2), Nil))),
            v("1101100001110110001000")
        );
        let second_item = cons(Number(2), cons(Number(3), Nil));
        let woosh = cons(Number(1), cons(second_item, cons(Number(4), Nil)));
        assert_eq!(
            mod_list(&woosh),
            v("1101100001111101100010110110001100110110010000")
        );
    }

    #[test]
    fn test_modem() {
        let cons = |a, b| NestedList::Cons(Box::new(a), Box::new(b));
        let num = |x| NestedList::Number(x);
        let nil = || NestedList::Nil;

        assert_eq!(dem_list(&v("010")), n(0));
        assert_eq!(dem_list(&v("0111000010000")), n(16));
        assert_eq!(dem_list(&v("00")), nil());
        assert_eq!(dem_list(&v("1101000")), cons(Number(0), Nil));

        let var0 = cons(Number(0), Nil);
        assert_eq!(var0, dem_list(&mod_list(&var0)));

        let var1 = cons(num(1), cons(num(2), cons(num(3), nil())));
        assert_eq!(var1, dem_list(&mod_list(&var1)));

        let var2 = cons(cons(num(1), num(2)), cons(nil(), nil()));
        assert_eq!(var2, dem_list(&mod_list(&var2)));
    }
}
