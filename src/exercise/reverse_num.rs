
///
/// 翻转数字
///
/// # Example
///
/// ```test
///
/// assert_eq!(-321, reverse(-123));
/// assert_eq!(0, reverse(2147483647));
///
/// ```
///
pub fn reverse(x: i32) -> i32 {

    use std::i32;
    let mut x = x;
    let mut result:Option<i32> = Some(0);

    loop {
        if x == 0 {
            break;
        }
        let tmp_num = x % 10;
        x = x / 10;

        result = result.unwrap().checked_mul(10);
        if result.is_none() {
            return 0i32;
        }

        result = tmp_num.checked_add(result.unwrap());
        if result.is_none() {
            return 0_i32;
        }
    }

    result.unwrap()
}