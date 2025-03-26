use std::io;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get the sizes of arrays a and b
    let first_line = lines.next().expect("No first line").expect("Error reading first line");
    let mut parts = first_line.split_whitespace();
    let a_size: usize = parts.next().unwrap().parse().unwrap();
    let b_size: usize = parts.next().unwrap().parse().unwrap();

    // Collect all remaining numbers into a vector
    let mut all_numbers = Vec::new();
    for line in lines {
        let line = line.expect("Error reading line");
        all_numbers.extend(
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
        );
    }

    // Split the collected numbers into arrays a and b
    let a: Vec<_> = all_numbers.drain(0..a_size).collect();
    let b: Vec<_> = all_numbers.drain(0..b_size).collect();

    // Compute the maximum score and print it
    let result = max_score(&a, &b);
    println!("{}", result);
}

/// Computes the maximum score using dynamic programming.
///
/// The function uses a DP table where `f[i][j]` represents the maximum score
/// achievable using the first `i` elements of `b` and exactly `j` elements of `a`.
/// The final result is `f[b.len()][4]`, which uses all elements of `b` and exactly 4 elements of `a`.
fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1];
    f[0][0] = 0;

    for i in 1..=n {
        for j in 0..=4 {
            // Initialize with the value from the previous row (not using the current b[i-1])
            f[i][j] = f[i - 1][j];
            // If we can take one more element from a, compute the alternative value
            if j > 0 {
                let val = f[i - 1][j - 1] + (a[j - 1] as i64 * b[i - 1] as i64);
                f[i][j] = f[i][j].max(val);
            }
        }
    }

    f[n][4]
}