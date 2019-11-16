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
        let items = match (third_bit, fourth_bit, fifth_bit) {
            (false, _, _) => (iter.next(), None, None),
            (_, false, _) => (iter.next(), iter.next(), None),
            (_, _, false) => (iter.next(), iter.next(), iter.next()),
            _ => {
                return false;
            }
        };

        return match items {
            (Some(&i1), None, None) if !third_bit => {
                if is_10x(i1) {
                    continue;
                }
                false
            }
            (Some(&i1), Some(&i2), None) if !fourth_bit => {
                if is_10x(i1) && is_10x(i2) {
                    continue;
                }
                false
            }
            (Some(&i1), Some(&i2), Some(&i3)) if !fifth_bit => {
                if is_10x(i1) && is_10x(i2) && is_10x(i3) {
                    continue;
                }
                false
            }
            _ => {
                false
            }
        };
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::simple::is_unicode;

    #[test]
    fn something() {
        //        let raw_chars: Vec<char> = thread_rng().sample_iter(&Standard).take(10000).collect();
        //        let raw_bytes = String::from_iter(raw_chars).into_bytes();
        let slice = include_bytes!("../benches/unicode_test_set.txt");

        assert_eq!(is_unicode(slice), true);
    }

    #[test]
    fn it_works() {
        let bytes = [197, 130, 1];
        assert_eq!(is_unicode(&bytes), true);
    }

    #[test]
    fn it_fails() {
        let bytes = [235, 140, 4];
        assert_eq!(is_unicode(&bytes), false);
    }
}
