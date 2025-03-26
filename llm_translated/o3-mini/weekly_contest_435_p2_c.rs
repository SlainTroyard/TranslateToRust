use std::io::{self, BufRead};

/// Compute maximum distance according to the algorithm in the original C code.
///
/// For every step in the string, update the x and y coordinates according to the direction.
/// Then, compute the current maximum distance possible by taking the minimum of:
///    - (|x| + |y| + 2*k) and the total number of steps so far (i+1).
/// Finally, update the overall answer.
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    // Use 'steps' to track the number of moves so far (i+1)
    let mut steps = 0;
    
    // Iterate over each character in the string.
    for c in s.chars() {
        steps += 1;
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => (), // Ignore any characters that are not directional.
        }
        // Calculate the current maximum potential distance:
        // total = |x| + |y| plus k * 2 (as in original C code)
        let current_total = x.abs() + y.abs() + k * 2;
        // The step limit is 'steps' (i+1 equivalent) and we take the minimum.
        let current_max = current_total.min(steps);
        // Update overall answer with the maximum seen so far.
        ans = ans.max(current_max);
    }
    
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We create a buffered reader for standard input 
    let stdin = io::stdin();
    let mut input_line = String::new();

    // Read one line of input
    if stdin.lock().read_line(&mut input_line)? == 0 {
        // If nothing is read, print an error message and exit.
        eprintln!("Error reading input");
        return Ok(());
    }
    
    // Split the input line into tokens.
    // The input format is expected to be: <s> <k>
    let tokens: Vec<&str> = input_line.split_whitespace().collect();
    if tokens.len() < 2 {
        eprintln!("Error reading input");
        return Ok(());
    }
    
    // The first token is the string, second token is the integer k.
    let s = tokens[0];
    let k: i32 = tokens[1].parse()?;

    // Compute the result using the max_distance function.
    let result = max_distance(s, k);
    
    // Output the result exactly as required.
    println!("{}", result);
    
    Ok(())
}