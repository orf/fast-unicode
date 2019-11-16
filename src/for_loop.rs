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
        let fourth_bit = get_bit_at(number, 4);
        let fifth_bit = get_bit_at(number, 3);

        if third_bit && fourth_bit && fifth_bit {
            return false;
        }

        for bit in &[third_bit, fourth_bit, fifth_bit] {
            let byte = iter.next();

            if byte.filter(|&&b| is_10x(b)).is_none() {
                return false;
            }

            if !bit {
                break;
            }
        }
    }
    true
}
