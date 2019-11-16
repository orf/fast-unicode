// from https://helloacm.com/how-to-validate-utf-8-encoding-the-simple-utf-8-validation-algorithm/

fn get_bit_at(input: u8, n: u8) -> bool {
    input & (1 << n) != 0
}

fn is_10x(input: u8) -> bool {
    let bit1 = get_bit_at(input, 7);
    let bit2 = get_bit_at(input, 6);
    bit1 && !bit2
}

pub fn is_unicode(slice: &[u8]) -> bool {
    let mut idx = 0;
    while idx < slice.len() {
        let number = slice[idx];
        // 0xxxxxxx
        let first_bit = get_bit_at(number, 7);
        if !first_bit {
            idx += 1;
            continue;
        }
        // 110xxxxx 10xxxxxx
        let second_bit = get_bit_at(number, 6);
        if !second_bit {
            return false;
        }
        let third_bit = get_bit_at(number, 5);
        if !third_bit {
            // 110xxxxx 10xxxxxx
            if idx + 1 < slice.len() {
                if is_10x(slice[idx + 1]) {
                    idx += 2;
                    continue;
                }
                return false;
            } else {
                return false;
            }
        }
        let fourth_bit = get_bit_at(number, 4);
        if !fourth_bit {
            // 1110xxxx 10xxxxxx 10xxxxxx
            if idx + 2 < slice.len() {
                if slice[idx + 1..idx + 3].iter().all(|f| is_10x(*f)) {
                    idx += 3;
                    continue;
                }
                return false;
            } else {
                return false;
            }
        }
        let fifth_bit = get_bit_at(number, 3);
        if fifth_bit {
            return false;
        }
        if idx + 3 < slice.len() {
            if slice[idx + 1..idx + 4].iter().all(|f| is_10x(*f)) {
                idx += 4;
                continue;
            }
            return false;
        } else {
            return false;
        }
    }
    true
}
