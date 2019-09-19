use std::ops::Range;

///
/// ## 第一种解法
///
///
///
pub fn convert(s: String, num_rows: i32) -> String {
    let mut result = String::new();
    let seq: Vec<char> = s.chars().collect();
    let length = s.len();
    if length < 3 || num_rows < 2{
        return s;
    }

    for i in 0..num_rows {
        let mut j:i32 = 0;
        loop {
            let mut idx:usize = 0;
            if i > 0 && i < num_rows - 1 {
                let k:i32 = j/2;
                let tail = k * (2 * num_rows - 2) + num_rows - 1;
                let offset = (num_rows - i - 1) * (-1_i32).pow(j as u32);
                idx = (tail - offset) as usize;
            } else {
                idx = (j * (num_rows + num_rows - 2) + i) as usize;
            }

            if idx >= length {
                break;
            }
            result.push(seq[idx]);

            j += 1;
        }
    }

    result
}


///
/// ## leetcode中的优秀解法
///
///
fn convert1(s: String, num_rows: i32) -> String {
    if num_rows<=1 { return s; }
    let s = s.as_bytes();
    let mut ret = String::with_capacity(s.len());
    for n in 0..num_rows {
        let mut idx = 0;
        for &i in [n].iter().chain([(num_rows-n-1)*2, 2*n].iter().cycle()) {
            println!("{},{}", n, i);
            if idx+i != idx || idx==0 {
                idx += i;
                if idx as usize>=s.len() { break; }
                ret.push(s[idx as usize] as char);
            }
        }
    }
    ret
}