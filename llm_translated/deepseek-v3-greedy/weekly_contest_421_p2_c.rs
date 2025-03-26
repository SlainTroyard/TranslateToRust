use std::io::{self, Write};

fn length_after_transformations(s: &str, t: i32) -> i32 {
    let mut lst = [0; 26];
    let modulo = 1_000_000_007;

    // Count the frequency of each character in the string
    for c in s.chars() {
        lst[(c as u8 - b'a') as usize] += 1;
    }

    // Perform the transformations
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim();

    let mut t_input = String::new();
    io::stdin().read_line(&mut t_input).expect("Failed to read input");
    let t: i32 = t_input.trim().parse().expect("Invalid input for t");

    let result = length_after_transformations(s, t);
    println!("{}", result);
}