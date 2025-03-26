// Problem: Weekly Contest 415 Problem 2

use std::io;

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1];

    f[0][0] = 0;
    for i in 1..=n {
        for j in 0..=4 {
            f[i][j] = f[i - 1][j];
            if j > 0 {
                f[i][j] = f[i][j].max(f[i - 1][j - 1] + (a[j - 1] as i64) * (b[i - 1] as i64));
            }
        }
    }
    f[n][4]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the sizes of arrays a and b
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let a_size: usize = iter.next().ok_or("Invalid input")?.parse()?;
    let b_size: usize = iter.next().ok_or("Invalid input")?.parse()?;

    // Read array a
    let mut a = vec![0; a_size];
    for i in 0..a_size {
        a[i] = read_int()?;
    }

    // Read array b
    let mut b = vec![0; b_size];
    for i in 0..b_size {
        b[i] = read_int()?;
    }

    // Calculate and print the result
    let result = max_score(&a, &b);
    println!("{}", result);

    Ok(())
}

// Helper function to read an integer from stdin
fn read_int() -> Result<i32, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().parse().map_err(|e| e.into())
}