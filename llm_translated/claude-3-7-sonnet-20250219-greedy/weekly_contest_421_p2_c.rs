use std::io::{self, BufRead};

const MOD: i32 = 1_000_000_007;

fn length_after_transformations(s: &str, mut t: i32) -> i32 {
    // Count frequency of each letter in the string
    let mut lst = [0; 26];
    for c in s.bytes() {
        lst[(c - b'a') as usize] += 1;
    }

    // Apply transformations t times
    while t > 0 {
        let z = lst[25]; // Save the count of 'z'
        
        // Shift all letters (except 'a' and 'b') one position to the right
        for i in (2..=25).rev() {
            lst[i] = lst[i - 1];
        }
        
        // Special handling for 'b' and 'a'
        lst[1] = (lst[0] + z) % MOD;
        lst[0] = z;
        
        t -= 1;
    }
    
    // Calculate the total length after transformations
    let mut ans = 0;
    for i in 0..26 {
        ans = (ans + lst[i]) % MOD;
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the string s
    let s = lines.next().unwrap().unwrap();
    
    // Read the integer t
    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    
    // Calculate and print the result
    print!("{}", length_after_transformations(&s, t));
}