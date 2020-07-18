#[derive(Debug)]
pub enum NestedList {
    Number(i64),
    List(Vec<NestedList>),
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

fn demodulate(iter: &mut Iterator<Item = bool>) -> Result<i64, ()> {
    let negative = match iter.next() {
        Some(val) => val,
        None => return Err(()),
    };
    match iter.next() {
        Some(val) => val,
        None => return Err(()),
    };

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

    fn v(s: &str) -> Vec<bool> {
        s.chars()
            .map(|c| if c == '1' { true } else { false })
            .collect()
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
}
