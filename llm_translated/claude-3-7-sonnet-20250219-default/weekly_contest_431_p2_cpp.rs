use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn calculate_score(s: String) -> i64 {
        let mut stacks: [Vec<usize>; 26] = std::array::from_fn(|_| Vec::new());
        let mut ans: i64 = 0;

        for (i, ch) in s.chars().enumerate() {
            let c = (ch as u8 - b'a') as usize;
            
            // Check if there's a matching character (25 - c is the complement, like 'a' and 'z')
            if !stacks[25 - c].is_empty() {
                // We found a match, calculate score as difference in positions
                let pos = stacks[25 - c].pop().unwrap();
                ans += (i - pos) as i64;
            } else {
                // No match found, push current position to stack
                stacks[c].push(i);
            }
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    // Read the input string
    if let Some(Ok(s)) = iterator.next() {
        let solution = Solution;
        println!("{}", Solution::calculate_score(s));
    }
}