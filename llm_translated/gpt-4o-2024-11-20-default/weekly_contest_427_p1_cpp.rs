use std::io;

struct Solution;

impl Solution {
    pub fn construct_transformed_array(a: Vec<i32>) -> Vec<i32> {
        let n = a.len() as i32;
        let mut res = Vec::with_capacity(n as usize);
        for i in 0..n {
            // Calculate the transformed index using modulus and avoid negative indices
            let transformed_index = (i + a[i as usize] % n + n) % n;
            res.push(a[transformed_index as usize]);
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // Read the size of the array
    stdin.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read the elements of the array
    input.clear();
    stdin.read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    assert!(a.len() == n, "The size of the array does not match the specified size.");

    let solution = Solution;
    let transformed_array = solution.construct_transformed_array(a);

    // Output the transformed array
    println!(
        "{}",
        transformed_array
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}