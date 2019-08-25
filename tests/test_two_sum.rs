extern crate learn_rust;

#[cfg(test)]
pub mod tests {
    use learn_rust::exercise::two_sum::two_sum;

    #[test]
    pub fn test_two_sum() {
        let nums:Vec<usize> = vec![2, 7, 11, 15];
        let n:Vec<usize> = two_sum(nums, 9_usize);

        assert_eq!(n, vec![0,1]);
    }
}