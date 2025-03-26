// Problem: Weekly Contest 427 Problem 1
use std::io;

struct Solution;

impl Solution {
    pub fn construct_transformed_array(a: Vec<i32>) -> Vec<i32> {
        let n = a.len() as i32; // Convert to i32 for arithmetic consistency
        let mut res = vec![0; n as usize];
        for i in 0..n {
            // Calculate the transformed index and assign the value
            res[i as usize] = a[((i + a[i as usize] % n + n) % n) as usize];
        }
        res
    }
}

fn main() {
    // Create a new instance of the solution
    let solution = Solution;

    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap(); // Read the size of the array

    // Read the array elements
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(a.len(), n);

    // Call the construct_transformed_array function
    let transformed_array = solution.construct_transformed_array(a);

    // Output the transformed array
    let output: String = transformed_array
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", output);
}