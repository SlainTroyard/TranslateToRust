use std::io;

/// Translates the C stack implementation to idiomatic Rust
/// using Vec which already provides all stack functionality needed
fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    // One vec (stack) for each letter of the alphabet
    let mut stacks: Vec<Vec<usize>> = vec![Vec::with_capacity(len); 26];
    let mut ans: i64 = 0;
    
    for (i, c) in s.chars().enumerate() {
        let c = (c as u8 - b'a') as usize;
        let matching_char = 25 - c; // Complementary character index (a->z, b->y, etc.)
        
        if !stacks[matching_char].is_empty() {
            // Found a match - calculate score and pop
            let top_pos = stacks[matching_char].pop().unwrap();
            ans += (i as i64) - (top_pos as i64);
        } else {
            // No match - push current position
            stacks[c].push(i);
        }
    }
    
    ans
}

fn main() -> io::Result<()> {
    // The original C code handles strings of up to 100000 characters
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    // Trim whitespace and newline characters
    let s = input.trim();
    
    let result = calculate_score(s);
    println!("{}", result);
    
    Ok(())
}