use std::cmp;
use std::io::{self, BufRead};

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1]; // f[i][j] represents the max score using the first i elements of b and j elements of a.

    f[0][0] = 0; // Base case: No elements used, score is 0.

    for i in 1..=n {
        for j in 0..=4 {
            // Transition 1: Skip the current element of b, score remains the same.
            f[i][j] = f[i - 1][j];
            
            if j > 0 {
                // Transition 2: Use the current element of b with the jth-1 element of a.
                f[i][j] = cmp::max(f[i][j], f[i - 1][j - 1] + a[j - 1] as i64 * b[i - 1] as i64);
            }
        }
    }

    f[n][4] // The result is the maximum score using exactly 4 elements of `a` with elements of `b`
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the sizes of arrays a and b
    let sizes = lines.next().unwrap().unwrap();
    let mut sizes_iter = sizes.split_whitespace();
    let a_size: usize = sizes_iter.next().unwrap().parse().unwrap();
    let b_size: usize = sizes_iter.next().unwrap().parse().unwrap();

    // Read the array `a`
    let a_line = lines.next().unwrap().unwrap();
    let a: Vec<i32> = a_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    assert_eq!(a.len(), a_size); // Ensure input matches specified size

    // Read the array `b`
    let b_line = lines.next().unwrap().unwrap();
    let b: Vec<i32> = b_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    assert_eq!(b.len(), b_size); // Ensure input matches specified size

    // Calculate the maximum score
    let result = max_score(&a, &b);

    // Output the result
    println!("{}", result);
}