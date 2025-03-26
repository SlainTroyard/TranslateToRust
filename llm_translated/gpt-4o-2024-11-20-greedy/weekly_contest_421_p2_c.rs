use std::io::{self, Read};

const MOD: i32 = 1_000_000_007;

fn length_after_transformations(s: &str, t: i32) -> i32 {
    let mut lst = [0; 26];
    let mut ans = 0;

    // Count occurrences of each character in the string
    for ch in s.chars() {
        lst[(ch as u8 - b'a') as usize] += 1;
    }

    // Perform t transformations
    let mut t = t;
    while t > 0 {
        let mut z = lst[25];
        for i in (1..26).rev() {
            lst[i] = lst[i - 1];
        }
        lst[1] = (lst[0] + z) % MOD;
        lst[0] = z;
        t -= 1;
    }

    // Calculate the final answer
    for count in lst.iter() {
        ans = (ans + count) % MOD;
    }

    ans
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Split input into lines
    let mut lines = input.lines();

    // First line is the string `s`
    let s = lines.next().expect("Missing input string");

    // Second line is the integer `t`
    let t: i32 = lines
        .next()
        .expect("Missing input integer")
        .trim()
        .parse()
        .expect("Failed to parse integer");

    // Compute the result
    let result = length_after_transformations(s, t);

    // Print the result to stdout
    println!("{}", result);
}