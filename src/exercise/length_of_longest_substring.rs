use std::collections::HashMap;
use std::cmp::max;

pub fn length_of_longest_substring(s:String) -> i32 {
    let mut max_len:usize = 0;
    let mut tmp_len:usize = 0;

    let mut tmp_hash: HashMap<char, usize> = HashMap::new();
    for (k, v) in s.chars().enumerate() {
        match tmp_hash.get(&v) {
            Some(idx) => {
                let delta = k - idx;
                if delta <= tmp_len {
                    max_len = max(max_len, tmp_len);
                    tmp_len = delta;
                } else {
                    tmp_len += 1;
                }
            },
            None => tmp_len += 1
        }

        tmp_hash.insert(v, k);
    }

    max_len = max(max_len, tmp_len);
    max_len as i32
}