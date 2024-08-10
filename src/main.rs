struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        return x.to_string().chars().rev().collect::<String>() == x.to_string()
        
    }
}


fn main() {
    let a = 121;
    let b = -121;
    let c = 10;

    let ra = Solution::is_palindrome(a);
    let rb = Solution::is_palindrome(b);
    let rc = Solution::is_palindrome(c);

    println!("ra {ra}, rb {rb}, rc {rc}");
}