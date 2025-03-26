use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();
    println!("{}", count_substrings(s));
}

fn count_substrings(s: &str) -> i64 {
    let mut ans = 0i64;
    // f[m][rem] tracks count of numbers with remainder 'rem' modulo m (1-9)
    let mut f = [[0i64; 9]; 10]; // [modulus][remainder], modulus 0 unused
    
    for c in s.chars() {
        let d = c.to_digit(10).expect("Input contains non-digit characters") as u32;
        
        // Update counts for each modulus from 1 to 9
        for m in 1..=9 {
            let m_usize = m as usize;
            let mut new_counts = [0i64; 9];
            
            // New single-digit number formed by current digit
            let remainder = (d % m) as usize;
            new_counts[remainder] = 1;
            
            // Add counts from previous numbers by extending them with current digit
            for rem in 0..m {
                let new_rem = ((rem * 10 + d) % m) as usize;
                new_counts[new_rem] += f[m_usize][rem as usize];
            }
            
            f[m_usize] = new_counts;
        }
        
        // Add count of numbers divisible by current digit (modulus d)
        let d_mod = d as usize;
        ans += f[d_mod][0];
    }
    
    ans
}