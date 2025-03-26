// Problem: Weekly Contest 421 Problem 2
use std::io;
use std::io::Read;

fn length_after_transformations(s: &str, t: i32) -> i32 {
    let mut lst: [i32; 26] = [0; 26];
    for &byte in s.as_bytes() {
        lst[(byte - b'a') as usize] += 1;
    }

    let mut t_mut = t;
    while t_mut > 0 {
        let mut z = lst[25];
        for i in (2..=25).rev() {
            lst[i] = lst[i - 1];
        }
        lst[1] = (lst[0] + z) % 1000000007;
        lst[0] = z;
        t_mut -= 1;
    }

    let mut ans: i32 = 0;
    for i in 0..26 {
        ans = (ans + lst[i]) % 1000000007;
    }
    ans
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim();

    let mut t_str = String::new();
    io::stdin().read_line(&mut t_str)?;
    let t: i32 = t_str.trim().parse().unwrap();

    let result = length_after_transformations(s, t);
    println!("{}", result);

    Ok(())
}