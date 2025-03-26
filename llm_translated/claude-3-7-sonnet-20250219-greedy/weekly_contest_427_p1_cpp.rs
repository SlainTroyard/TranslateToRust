// Problem: Weekly Contest 427 Problem 1
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn construct_transformed_array(a: &Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut res = vec![0; n];
        for i in 0..n {
            // Calculate the index with the same modular arithmetic as in C++
            // We need to add n before taking modulo to handle negative values correctly
            let index = ((i as i32 + a[i] % n as i32 + n as i32) % n as i32) as usize;
            res[i] = a[index];
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Input the elements of the array
    let a: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Call the constructTransformedArray function
    let solution = Solution;
    let transformed_array = Solution::construct_transformed_array(&a);
    
    // Output the transformed array
    for (i, num) in transformed_array.iter().enumerate() {
        print!("{}", num);
        if i < transformed_array.len() - 1 {
            print!(" ");
        }
    }
    println!();
}