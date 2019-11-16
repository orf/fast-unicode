fn get_bit_at(input: u8, n: u8) -> bool {
    input & (1 << n) != 0
}

fn is_10x(input: u8) -> bool {
    let bit1 = get_bit_at(input, 7);
    let bit2 = get_bit_at(input, 6);
    bit1 && !bit2
}

// from https://helloacm.com/how-to-validate-utf-8-encoding-the-simple-utf-8-validation-algorithm/

pub fn is_unicode(slice: &[u8]) -> bool {
    let mut iter = slice.iter();
    while let Some(&number) = iter.next() {
        // 0xxxxxxx
        let first_bit = get_bit_at(number, 7);
        if !first_bit {
            // Skip 1
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
            return match &iter.next() {
                Some(&i) => {
                    if is_10x(i) {
                        continue;
                    }
                    false
                }
                None => false,
            };
        }
        let fourth_bit = get_bit_at(number, 4);
        if !fourth_bit {
            // 1110xxxx 10xxxxxx 10xxxxxx
            return match (iter.next(), iter.next()) {
                (Some(&i1), Some(&i2)) => {
                    if is_10x(i1) && is_10x(i2) {
                        continue;
                    }
                    false
                }
                _ => false,
            };
        }
        let fifth_bit = get_bit_at(number, 3);
        if fifth_bit {
            return false;
        }
        return match (iter.next(), iter.next(), iter.next()) {
            (Some(&i1), Some(&i2), Some(&i3)) => {
                if is_10x(i1) && is_10x(i2) && is_10x(i3) {
                    continue;
                }
                false
            }
            _ => false,
        };
    }
    true
}
