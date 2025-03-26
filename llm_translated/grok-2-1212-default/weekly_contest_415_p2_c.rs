use std::io::{self, BufRead};

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1];

    for i in 0..=n {
        for j in 0..=4 {
            f[i][j] = i64::MIN;
        }
    }
    f[0][0] = 0;
    for i in 1..=n {
        for j in 0..=4 {
            f[i][j] = f[i - 1][j];
            if j > 0 {
                f[i][j] = f[i][j].max(f[i - 1][j - 1] + i64::from(a[j - 1]) * i64::from(b[i - 1]));
            }
        }
    }
    f[n][4]
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read aSize and bSize
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let a_size: usize = iter.next().unwrap().parse().unwrap();
    let b_size: usize = iter.next().unwrap().parse().unwrap();

    // Read array a
    let a_line = lines.next().unwrap()?;
    let a: Vec<i32> = a_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Read array b
    let b_line = lines.next().unwrap()?;
    let b: Vec<i32> = b_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Calculate and print the result
    let result = max_score(&a, &b);
    println!("{}", result);

    Ok(())
}