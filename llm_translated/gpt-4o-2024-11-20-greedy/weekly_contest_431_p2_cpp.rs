use std::io::{self, Write};
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn calculate_score(s: String) -> i64 {
        // Create a vector of 26 stacks (VecDeque is used to emulate stack behavior in Rust)
        let mut stacks: Vec<VecDeque<usize>> = vec![VecDeque::new(); 26];
        let mut ans: i64 = 0;

        for (i, ch) in s.chars().enumerate() {
            // Calculate the character index relative to 'a'
            let c = (ch as u8 - b'a') as usize;

            // Check the complementary stack indexed by 25 - c
            if let Some(&top) = stacks[25 - c].front() {
                // If it is not empty, add the difference i - top to ans
                ans += (i - top) as i64;
                // Remove the top element from the complementary stack
                stacks[25 - c].pop_front();
            } else {
                // Otherwise, push the current index to the corresponding stack
                stacks[c].push_front(i);
            }
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input)?;

    // Remove trailing newline characters
    let s = input.trim().to_string();

    // Instantiate the Solution struct and calculate the score
    let solution = Solution;
    let result = solution.calculate_score(s);

    // Print the result to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result)?;

    Ok(())
}