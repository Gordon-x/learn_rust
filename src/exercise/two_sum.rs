// 假设全部为正整数
//给定 nums = [2, 7, 11, 15], target = 9
//
//因为 nums[0] + nums[1] = 2 + 7 = 9
//所以返回 [0, 1]
//
use std::collections::HashMap;

pub fn two_sum(nums: Vec<usize>, target: usize) -> Vec<usize> {

    let mut res:Vec<usize> = vec![];
    let mut h: HashMap<usize, usize> = HashMap::new();
    for (k, n) in nums.iter().enumerate() {
        let key = target - n;
        match h.get(&key) {
            Some(v) => {
                res.push(*v);
                res.push(k);
                break;
            },
            None => {
                h.insert(*n, k);
            }
        }
    }
    res
}