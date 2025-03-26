// Problem: Weekly Contest 417 Problem 1 - Translated from C to Rust
use std::io;
use std::io::Read;

fn kth_character(k: i32) -> char {
    let mut l: Vec<i32> = vec![0; (k + 1) as usize];
    let mut a = 0;
    loop {
        for i in 0..(1 << a) {
            let j = 1 << a;
            let index = (i + j) as usize;
            if index < l.len() {
                l[index] = l[i as usize] + 1;
                if (i + j) >= (k - 1) {
                    return (97 + l[(k - 1) as usize] % 26) as u8 as char;
                }
            } else {
                // Avoid out-of-bounds access by stopping the inner loop
                return (97 + l[(k - 1) as usize] % 26) as u8 as char;
            }
        }
        a += 1;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let k: i32 = input.trim().parse()?;
    println!("{}", kth_character(k));
    Ok(())
}