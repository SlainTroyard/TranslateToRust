use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn construct_transformed_array(a: &Vec<i32>) -> Vec<i32> {
        let n = a.len() as i32;
        let mut res = vec![0; a.len()];
        for i in 0..a.len() {
            let i_i32 = i as i32;
            let index = (i_i32 + a[i] % n + n) % n;
            res[i] = a[index as usize];
        }
        res
    }
}

fn main() {
    let solution = Solution {};

    // Input the size of the array
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input");

    // Input the elements of the array
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("Failed to read line");
    let a: Vec<i32> = a_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Call the constructTransformedArray function
    let transformed_array = solution.construct_transformed_array(&a);

    // Output the transformed array
    for &num in &transformed_array {
        print!("{} ", num);
    }
    println!();
}