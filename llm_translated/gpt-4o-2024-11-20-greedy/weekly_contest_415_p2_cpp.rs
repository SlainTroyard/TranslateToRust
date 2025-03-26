use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_score(a: Vec<i64>, b: Vec<i64>) -> i64 {
        // Initialize the array `f` with 5 elements, where the first is 0 and the rest are set to a very small value.
        let mut f = [0, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2, i64::MIN / 2];
        
        // Iterate over each element `y` in vector `b`.
        for &y in &b {
            // Update the array `f` in reverse order to avoid overwriting values prematurely.
            for j in (0..4).rev() {
                f[j + 1] = max(f[j + 1], f[j] + a[j] * y);
            }
        }
        
        // Return the maximum score stored in `f[4]`.
        f[4]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the sizes of the two arrays `a` and `b`.
    let first_line = lines.next().unwrap().unwrap();
    let mut sizes = first_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let a_size = sizes.next().unwrap();
    let b_size = sizes.next().unwrap();

    // Read the elements of array `a`.
    let second_line = lines.next().unwrap().unwrap();
    let a: Vec<i64> = second_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

    // Read the elements of array `b`.
    let third_line = lines.next().unwrap().unwrap();
    let b: Vec<i64> = third_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

    // Ensure the input sizes match the expected sizes.
    assert_eq!(a.len(), a_size);
    assert_eq!(b.len(), b_size);

    // Create an instance of the solution and compute the result.
    let solution = Solution;
    let result = solution.max_score(a, b);

    // Print the result to stdout.
    println!("{}", result);
}