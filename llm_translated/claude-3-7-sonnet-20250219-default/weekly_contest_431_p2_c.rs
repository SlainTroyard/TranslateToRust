fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim().to_string();
    
    println!("{}", calculate_score(&s));
}

// Main solution function
fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    // One stack for each letter
    let mut stacks: Vec<Vec<usize>> = vec![Vec::with_capacity(len); 26];
    let mut ans: i64 = 0;
    
    for (i, ch) in s.chars().enumerate() {
        let c = (ch as u8 - b'a') as usize;
        
        // Check for a match with the complementary character (z matches with a, y with b, etc.)
        let complement_idx = 25 - c;
        if !stacks[complement_idx].is_empty() {
            // Found a match - calculate score and pop
            let top_pos = stacks[complement_idx].pop().unwrap();
            ans += (i as i64) - (top_pos as i64);
        } else {
            // No match - push current position
            stacks[c].push(i);
        }
    }
    
    ans
}