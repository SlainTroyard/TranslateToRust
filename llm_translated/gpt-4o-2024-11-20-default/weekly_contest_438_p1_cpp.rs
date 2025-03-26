use std::io;

struct Solution;

impl Solution {
    /// This function replicates the behavior of `hasSameDigits` from the C++ version.
    /// It repeatedly reduces a string of digits by summing the adjacent pairs modulo 10
    /// until the size of the resulting string is 2, and checks if the two digits are the same.
    fn has_same_digits(s: String) -> bool {
        let mut s = s;
        while s.len() > 2 {
            let mut t = String::new();
            let chars: Vec<char> = s.chars().collect();
            
            for i in 0..chars.len() - 1 {
                let x = chars[i].to_digit(10).unwrap(); // Convert char to digit
                let y = chars[i + 1].to_digit(10).unwrap();
                t.push(((x + y) % 10).to_string().chars().next().unwrap());
            }

            s = t; // Replace with the new reduced string
        }

        let chars: Vec<char> = s.chars().collect();
        chars[0] == chars[1]
    }
}

fn main() {
    // Standard I/O handling to match the C++ program
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string(); // Read input as a trimmed string

    let solution = Solution;
    let result = solution.has_same_digits(s);

    // Output result as a boolean (0/1) matching the C++ output style
    println!("{}", result as i32);
}