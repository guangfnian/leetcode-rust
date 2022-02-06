struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let max: i64 = (1 << 31) - 1;
        let min: i64 = -(1 << 31);
        let mut started = false;
        let mut ans = 0;
        let mut negative = false;
        for c in s.chars() {
            match c {
                ' ' => {
                    if started {
                        break;
                    }
                }
                '0'..='9' => {
                    started = true;
                    ans = ans * 10 + c.to_digit(10).unwrap() as i64;
                    if ans > max {
                        ans = max + 1
                    }
                }
                '-' => {
                    if started {
                        break;
                    }
                    started = true;
                    negative = true;
                }
                '+' => {
                    if started {
                        break;
                    }
                    started = true;
                }
                _ => break,
            };
        }
        if negative {
            ans = -ans
        }
        if ans > max {
            ans = max
        }
        if ans < min {
            ans = min
        }
        ans as i32
    }
}

#[cfg(test)]
mod test {
    use crate::solution::string_to_integer_atoi::Solution;

    #[test]
    fn test() {
        let tests = [("42", 42), ("   -42", -42), ("4193 with words", 4193)];
        for test in tests {
            assert_eq!(test.1, Solution::my_atoi(String::from(test.0)));
        }
    }
}
