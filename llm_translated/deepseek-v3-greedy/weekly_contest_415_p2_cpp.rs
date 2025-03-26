use std::io;
use std::cmp::max;

fn main() {
    // Read the sizes of vectors a and b from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let sizes: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    let a_size = sizes[0];
    let b_size = sizes[1];

    // Read vector a from stdin
    let mut a = Vec::with_capacity(a_size);
    for _ in 0..a_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        a.push(num);
    }

    // Read vector b from stdin
    let mut b = Vec::with_capacity(b_size);
    for _ in 0..b_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        b.push(num);
    }

    // Calculate and print the maximum score
    let result = max_score(&a, &b);
    println!("{}", result);
}

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    // Initialize the DP array with the minimum possible value
    let mut f = [i64::MIN / 2; 5];
    f[0] = 0;

    // Iterate over each element in b
    for &y in b {
        // Update the DP array in reverse order to avoid overwriting
        for j in (0..4).rev() {
            f[j + 1] = max(f[j + 1], f[j] + a[j] as i64 * y as i64);
        }
    }

    // The result is the last element in the DP array
    f[4]
}