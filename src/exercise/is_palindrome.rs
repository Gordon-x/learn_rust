///
///
/// 判断回文数
///
/// # Example
///
/// ```test
/// assert_eq!(false, is_palindrome(-123));
/// assert_eq!(true, is_palindrome(0));
/// assert_eq!(true, is_palindrome(121));
/// assert_eq!(true, is_palindrome(124421));
/// assert_eq!(false, is_palindrome(1384));
///
/// ```
///
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {return false;}
    if x == 0 {return true;}

    let mut new_x = 0;
    let mut tmp_x = x;

    while tmp_x > 0 {
        new_x = new_x * 10 + tmp_x % 10;
        tmp_x /= 10;
    }

    new_x == x
}