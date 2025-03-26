use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn construct_transformed_array(a: &Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = a[((i + a[i] as usize) % n + n) % n];
        }
        res
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the size of the array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Input the elements of the array
    let a: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the construct_transformed_array function
    let transformed_array = Solution::construct_transformed_array(&a);

    // Output the transformed array
    for num in transformed_array {
        print!("{} ", num);
    }
    println!();

    Ok(())
}