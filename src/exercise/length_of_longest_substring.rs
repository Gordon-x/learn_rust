use std::collections::HashMap;
use std::cmp::max;

pub fn length_of_longest_substring(s:String) -> i32 {
    let mut rec = HashMap::<u8, usize>::new();
    let (mut max_len, mut start, mut tmp_len) = (0_i32, 0_i32, 0_i32);

    for (k, c) in s.bytes().enumerate() {
        match rec.get(&c) {
            Some(v) => {
                start = start.max(*v as i32);
                tmp_len = k as i32 - start;
            },
            None => {
                tmp_len += 1;
            }
        }
        max_len = max_len.max(tmp_len);
        rec.insert(c, k);
    }
    max_len
}