pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sum = 0i64;
        let mut sign = 1i64;
        let mut chars = s.trim().chars().peekable();

        if let Some(&f) = chars.peek() {
            if f == '-' {
                sign = -1;
                chars.next();
            } else if f == '+' {
                chars.next();
            }
        }

        for c in chars {
            if !c.is_numeric() {
                break;
            }
            let i = c.to_digit(10).unwrap() as i64;
            sum = sum * 10 + i;

            if sign == 1 && sum >= i32::MAX as i64 {
                return i32::MAX;
            }
            if sign == -1 && -sum <= i32::MIN as i64 {
                return i32::MIN;
            }
        }

        (sum * sign) as i32
    }
}
