use std::io::{self, Write};

// Function to calculate the length after transformations
fn length_after_transformations(s: &str, t: usize) -> i32 {
    let mut lst = [0; 26];
    let modulo = 1_000_000_007;

    // Count the frequency of each character in the string
    for c in s.chars() {
        lst[(c as u8 - b'a') as usize] += 1;
    }

    // Perform the transformations `t` times
    for _ in 0..t {
        let z = lst[25];
        for i in (1..26).rev() {
            lst[i] = lst[i - 1];
        }
        lst[1] = (lst[0] + z) % modulo;
        lst[0] = z;
    }

    // Calculate the total length after transformations
    let mut ans = 0;
    for &count in lst.iter() {
        ans = (ans + count) % modulo;
    }

    ans
}

fn main() {
    let mut s = String::new();
    let mut t_str = String::new();

    // Read the input string
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string();

    // Read the number of transformations
    io::stdin().read_line(&mut t_str).expect("Failed to read line");
    let t: usize = t_str.trim().parse().expect("Invalid number");

    // Calculate and print the result
    let result = length_after_transformations(&s, t);
    println!("{}", result);
}