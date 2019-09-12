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


    use learn_rust::exercise::substring::substring;
    #[test]
    pub fn test_sub() {
        let abc = String::from("abcabcbb");
        let length = substring(abc);

        println!("{}", length);

        assert_eq!(3, length);
    }

    use learn_rust::exercise::find_median::find_median_sorted_arrays;
    #[test]
    pub fn test_median() {
        let median = find_median_sorted_arrays(vec![1,3], vec![2]);
        assert_eq!(2.0, median);
    }
}