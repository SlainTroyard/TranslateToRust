use std::io::{self, BufRead};

fn calculate_score(s: &str) -> i64 {
    // Create an array of 26 stacks (vectors)
    let mut stacks: Vec<Vec<usize>> = vec![Vec::new(); 26];
    let mut ans: i64 = 0;
    
    // Iterate over each character with its index
    for (i, c) in s.chars().enumerate() {
        let c_index = (c as u8 - b'a') as usize;
        // Check stack for the mirror character: index = 25 - c_index
        if let Some(&top) = stacks[25 - c_index].last() {
            // If non-empty, add difference to ans and pop the top element
            ans += (i - top) as i64;
            stacks[25 - c_index].pop();
        } else {
            // Otherwise, push the current index onto the corresponding stack for c_index
            stacks[c_index].push(i);
        }
    }
    
    ans
}

fn main() -> io::Result<()> {
    // Set up input handling using stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first non-empty line
    let input_line = match lines.next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("Failed to read input");
            return Ok(());
        }
    };

    // Assume the entire input is a string token; trimming it to avoid whitespace issues.
    let s = input_line.trim();

    // Compute the score using the provided function
    let result = calculate_score(s);
    
    // Print the result
    println!("{}", result);

    Ok(())
}