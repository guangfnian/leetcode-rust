struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let max: i64 = (1 << 31) - 1;
        let min: i64 = -(1 << 31);
        let mut ans = 0_i64;
        let mut current = x.abs() as i64;
        while current > 0 {
            ans = ans * 10 + (current % 10);
            if ans > max || ans < min {
                return 0;
            }
            current /= 10;
        }
        if x < 0 {
            ans = -ans
        }
        ans as i32
    }
}

#[cfg(test)]
mod test {
    use crate::solution::reverse_integer::Solution;

    #[test]
    fn test() {
        let tests = [(123, 321), (-123, -321), (120, 21)];
        for test in tests.iter() {
            assert_eq!(test.1, Solution::reverse(test.0));
        }
    }
}
