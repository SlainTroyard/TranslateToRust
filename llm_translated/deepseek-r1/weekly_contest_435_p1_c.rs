use std::io;

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];  // Character count array for 'a'-'z'
    
    // Count occurrences of each character
    for c in s.chars() {
        let idx = (c as u8 - b'a') as usize;
        cnt[idx] += 1;
    }
    
    let mut c1 = 0;    // Track maximum odd count
    let mut c2 = 100;  // Track minimum even non-zero count
    
    for &count in cnt.iter() {
        if count % 2 == 1 {
            // Update maximum odd count
            if count > c1 {
                c1 = count;
            }
        } else if count != 0 {
            // Update minimum even count
            if count < c2 {
                c2 = count;
            }
        }
    }
    
    c1 - c2
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Extract first word to match C's scanf("%s") behavior
    let s = input.split_whitespace().next().unwrap_or("");
    
    println!("{}", max_difference(s));
}