use std::io;

struct Solution {}

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        for (i, c) in num.chars().enumerate() {
            let digit = c.to_digit(10).unwrap() as i32; //unwrap() because the problem guarantees a valid digit.
            if i % 2 == 0 {
                even_sum += digit;
            } else {
                odd_sum += digit;
            }
        }

        even_sum == odd_sum
    }
}

fn main() {
    let mut num = String::new();

    // Input the number as a string
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num = num.trim(); // Remove trailing newline

    // Create a Solution object and call the is_balanced function
    let solution = Solution {};
    let result = solution.is_balanced(num.to_string());

    // Output the result
    if result {
        println!("true");
    } else {
        println!("false");
    }
}