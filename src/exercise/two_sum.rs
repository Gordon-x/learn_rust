//
//给定 nums = [2, 7, 11, 15], target = 9
//
//因为 nums[0] + nums[1] = 2 + 7 = 9
//所以返回 [0, 1]
//
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut res:Vec<i32> = vec![];
    let mut h: HashMap<i32, i32> = HashMap::new();
    for (k, n) in nums.iter().enumerate() {
        let key = target - n;
        match h.get(&key) {
            Some(v) => {
                res.push(*v);
                res.push(k as i32);
                break;
            },
            None => {
                h.insert(*n, k as i32);
            }
        }
    }
    res
}