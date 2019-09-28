///
/// 字符串转换为数字
///
/// # Example
///
/// ```run
/// assert_eq!(my_atoi("1".to_string()), 1);
/// assert_eq!(my_atoi("   2".to_string()), 2);
/// assert_eq!(my_atoi("+".to_string()), 0);
/// assert_eq!(my_atoi("+-2".to_string()), 0);
/// assert_eq!(my_atoi(" 000023".to_string()), 23);
/// assert_eq!(my_atoi("wja 33".to_string()), 0);
/// assert_eq!(my_atoi(" -23kdk".to_string()), -23);
/// assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
/// ```
///
pub fn my_atoi(str: String) -> i32 {
    let bytes = str.as_bytes();

    let (mut start, mut idx) = (0, 0);
    let mut negative = false;

    let (mut num, mut symbol) = (false, false);
    let (zero, plus, sub, nine, space) = (b'0', b'+', b'-', b'9', b' ');

    while idx < bytes.len() {

        let ascii = bytes[idx];
        if (num || symbol) && (ascii == space || ascii == plus || ascii == sub) {
            break;
        }

        if ascii == space || ascii == plus || ascii == sub {
            start += 1;
            if ascii == plus || ascii == sub {
                symbol = true;
            }
            if ascii == sub {
                negative = true;
            }
        }

        if ascii >= zero && ascii <= nine {
            num = true;
        }

        if ascii != space && ascii != plus && ascii != sub && (ascii < zero || ascii > nine) {
            break;
        }

        idx += 1;
    }

    if start == idx {
        return 0;
    }

    use std::str::FromStr;

    if let Ok(v) = i32::from_str(&str[start..idx]) {
        if negative {
            return v.checked_mul(-1).unwrap_or(std::i32::MIN);
        }

        return v;
    }

    if negative {
        return std::i32::MIN;
    }
    return std::i32::MAX;
}