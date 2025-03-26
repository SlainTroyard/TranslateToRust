use std::io::{self, BufRead};

// Main function implementation
fn count_substrings(s: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut f = [[0i64; 9]; 10]; // Initialize counting array to 0
    
    for c in s.chars() {
        let d = (c as u8 - b'0') as usize; // Current digit
        
        for m in 1..10 {
            let mut nf = [0i64; 9]; // Temporary array for new counts
            nf[d % m] = 1;          // Single digit d mod m
            
            for rem in 0..m {
                // Update count: add current digit d to existing remainder rem to form new remainder
                nf[(rem * 10 + d) % m] += f[m][rem];
            }
            
            // Update f array
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }
        
        // Current digit is divisible by itself, increase result count
        if d > 0 { // Avoid division by zero
            ans += f[d][0];
        }
    }
    
    ans
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut input = String::new();
    
    if stdin.lock().read_line(&mut input).is_err() {
        eprintln!("Error reading input");
        std::process::exit(1);
    }
    
    // Remove whitespace
    let s = input.trim();
    
    // Call function to calculate result
    let result = count_substrings(s);
    
    // Output result
    println!("{}", result);
}