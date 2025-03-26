use std::io;
use std::collections::VecDeque;

const MOD: i32 = 1000000007;

// Transform the string and calculate the length after transformations
fn length_after_transformations(s: &str, t: i32) -> i32 {
    let mut lst = vec![0; 26]; // Array to store frequencies of each letter 'a' to 'z'
    
    // Count the frequency of each character in the string
    for ch in s.chars() {
        if let Some(index) = (ch as u8).checked_sub(b'a') {
            lst[index as usize] += 1;
        }
    }

    let mut t = t;
    while t > 0 {
        // Store the last value to be used after the shift
        let last = lst[25];
        
        // Perform the shifting operation
        for i in (1..26).rev() {
            lst[i] = lst[i - 1];
        }
        
        // Update the first and second positions based on the logic
        lst[1] = (lst[0] + last) % MOD;
        lst[0] = last;

        t -= 1;
    }

    // Calculate the sum of the array elements modulo MOD
    lst.into_iter().fold(0, |acc, x| (acc + x) % MOD)
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().expect("Failed to parse t");

    // Compute result using the function and print to stdout
    let result = length_after_transformations(&s, t);
    println!("{}", result);
}