use std::collections::HashMap;

struct Solution{}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        for (pos, num) in nums.iter().enumerate() {
            match mp.get(&(target-num)) {
                Some(pre) => return vec![*pre, pos as i32],
                None => mp.insert(*num, pos as i32),
            };
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use crate::solution::two_sum::Solution;

    #[test]
    fn test() {
        assert_eq!(vec![0,1], Solution::two_sum(vec![2,7,11,15], 9));
        assert_eq!(vec![1,2], Solution::two_sum(vec![3,2,4], 6));
        assert_eq!(vec![0,1], Solution::two_sum(vec![3,3], 6));
    }
}