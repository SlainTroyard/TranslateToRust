// Problem: Weekly Contest 417 Problem 1
use std::io;
use std::io::Read;

fn kth_character(k: i32) -> char {
    // Allocate a vector of i32 initialized to 0.  Using `usize` for indexing.
    let mut l: Vec<i32> = vec![0; (k + 1) as usize];

    let mut a: i32 = 0;
    loop {
        for i in 0..(1 << a) {
            let j: i32 = 1 << a;
            let index = (i + j) as usize;
            l[index] = l[i as usize] + 1;
            if (i + j) >= k - 1 {
                return char::from_u32((97 + l[(k - 1) as usize] % 26) as u32).unwrap();
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