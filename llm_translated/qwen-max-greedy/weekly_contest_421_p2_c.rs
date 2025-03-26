// Problem: Weekly Contest 421 Problem 2

use std::io;

fn length_after_transformations(s: &str, t: usize) -> usize {
    let mut lst = [0; 26];
    let mut ans = 0;

    // Count the frequency of each character in the string
    for c in s.chars() {
        if let Some(index) = c.to_ascii_lowercase().checked_sub('a' as u8) {
            lst[index as usize] += 1;
        }
    }

    // Perform the transformations
    for _ in 0..t {
        let z = lst[25];
        for i in (2..26).rev() {
            lst[i] = lst[i - 1];
        }
        lst[1] = (lst[0] + z) % 1_000_000_007;
        lst[0] = z;
    }

    // Calculate the final answer
    for &count in lst.iter() {
        ans = (ans + count) % 1_000_000_007;
    }

    ans
}

fn main() {
    let mut s = String::new();
    let mut t = String::new();

    // Read the input string
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");

    // Read the number of transformations
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line");

    // Parse the number of transformations
    let t: usize = t.trim().parse().expect("Please type a number!");

    // Trim the newline character from the input string
    let s = s.trim();

    // Calculate and print the result
    let result = length_after_transformations(s, t);
    println!("{}", result);
}