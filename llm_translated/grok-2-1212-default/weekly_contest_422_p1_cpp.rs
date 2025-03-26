use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        for (i, c) in num.chars().enumerate() {
            let digit = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                even_sum += digit as i32;
            } else {
                odd_sum += digit as i32;
            }
        }

        even_sum == odd_sum
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number as a string
    let num = lines.next().unwrap()?.trim().to_string();

    // Call the is_balanced function
    let result = Solution::is_balanced(num);

    // Output the result
    if result {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}