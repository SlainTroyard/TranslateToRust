use std::io;

struct Solution;

impl Solution {
    fn calculate_score(&self, s: String) -> i64 {
        // 26 stacks for each letter (a-z)
        let mut stk: [Vec<i64>; 26] = std::array::from_fn(|_| Vec::new());
        let mut ans: i64 = 0;
        
        for (i, c) in s.chars().enumerate() {
            let i = i as i64;
            // Calculate index in alphabet (0 for 'a', 1 for 'b', etc.)
            let c_idx = (c as u8 - b'a') as usize;
            // Calculate complementary character index (25-c: 'z' for 'a', 'y' for 'b', etc.)
            let complement_idx = 25 - c_idx;
            
            // If there's a matching complementary character in its stack
            if !stk[complement_idx].is_empty() {
                // Pop the position and add the distance to the answer
                let top_pos = stk[complement_idx].pop().unwrap();
                ans += i - top_pos;
            } else {
                // Otherwise, push the current position to its stack
                stk[c_idx].push(i);
            }
        }
        
        ans
    }
}

fn main() {
    // Read a single string from stdin
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string();
    
    // Create solution instance and calculate score
    let sol = Solution;
    let result = sol.calculate_score(s);
    
    // Print the result
    println!("{}", result);
}