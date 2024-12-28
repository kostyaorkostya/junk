struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        let digits = {
            let mut digits = Vec::new();
            let mut x = x;
            while x > 0 {
                digits.push(x % 10);
                x = x / 10;
            }
            digits
        };
        let n = digits.len();
        for i in 0..(n / 2) {
            if digits[i] != digits[n - 1 - i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("1 {}", Solution::is_palindrome(1));
    println!("13 {}", Solution::is_palindrome(13));
    println!("133 {}", Solution::is_palindrome(133));
    println!("1331 {}", Solution::is_palindrome(1331));
}
