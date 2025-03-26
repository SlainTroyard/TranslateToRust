use std::io::{self, Read};
use std::cmp;

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1];

    // Initialize base case
    f[0][0] = 0;
    
    // Dynamic programming approach
    for i in 1..=n {
        for j in 0..=4 {
            // Take the value from the previous row
            f[i][j] = f[i-1][j];
            
            // If we can use an element from array a
            if j > 0 {
                // Consider using the j-th element from a with the current element from b
                f[i][j] = cmp::max(f[i][j], f[i-1][j-1] + (a[j-1] as i64) * (b[i-1] as i64));
            }
        }
    }
    
    // Return the final answer
    f[n][4]
}

fn main() {
    // Read input size
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let a_size: usize = parts[0].parse().expect("Invalid input for a_size");
    let b_size: usize = parts[1].parse().expect("Invalid input for b_size");

    // Read array a
    let mut a = Vec::with_capacity(a_size);
    for _ in 0..a_size {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read array a");
        a.push(num.trim().parse::<i32>().expect("Invalid number in array a"));
    }

    // Read array b
    let mut b = Vec::with_capacity(b_size);
    for _ in 0..b_size {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read array b");
        b.push(num.trim().parse::<i32>().expect("Invalid number in array b"));
    }

    // Calculate and print the result
    println!("{}", max_score(&a, &b));
}