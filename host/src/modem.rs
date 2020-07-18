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
        res.push(num & mask == 0);
    }
}

fn demodulate(iter: &mut Iterator<Item=bool>) -> Result<i64,()> {
    let negative = match iter.next() { Some(val) => val, None => return Err(()) };
    match iter.next() { Some(val) => val, None => return Err(()) };

    let used_nibbles = 'outer: loop {
        for i in 0u64.. {
            if ! match iter.next() { Some(val) => val, None => return Err(()) } {
                break 'outer i;
            }
        }
    };

    let mut res = 0u64;
    for i in 0..(used_nibbles * 4) {
        if match iter.next() { Some(val) => val, None => return Err(()) } {
            res |= 1u64 << i;
        }
    }

    Ok(if negative {
        -(res as i64)
    } else {
        res as i64
    })
}
