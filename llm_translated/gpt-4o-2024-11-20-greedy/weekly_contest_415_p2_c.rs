use std::cmp::max;
use std::io::{self, BufRead};

fn max_score(a: &[i64], b: &[i64]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1];

    // Initialize the base case
    f[0][0] = 0;

    for i in 1..=n {
        for j in 0..=4 {
            // Carry forward the previous value
            f[i][j] = f[i - 1][j];
            if j > 0 {
                // Update the value if taking the current element
                f[i][j] = max(f[i][j], f[i - 1][j - 1] + a[j - 1] * b[i - 1]);
            }
        }
    }

    f[n][4]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the sizes of the arrays
    let first_line = lines.next().unwrap().unwrap();
    let mut sizes = first_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let a_size = sizes.next().unwrap();
    let b_size = sizes.next().unwrap();

    // Read the array `a`
    let second_line = lines.next().unwrap().unwrap();
    let a: Vec<i64> = second_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

    // Read the array `b`
    let third_line = lines.next().unwrap().unwrap();
    let b: Vec<i64> = third_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

    // Ensure the input sizes match the actual data
    assert_eq!(a.len(), a_size);
    assert_eq!(b.len(), b_size);

    // Compute the result
    let result = max_score(&a, &b);

    // Print the result
    println!("{}", result);
}