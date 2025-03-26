use std::io::{self, BufRead, Write};

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![-i64::MAX; 5]; n + 1];

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

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Read the first line to get the sizes of arrays a and b
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
    let mut iter = input.split_whitespace();
    let a_size: usize = iter.next().unwrap().parse().expect("Invalid input for aSize");
    let b_size: usize = iter.next().unwrap().parse().expect("Invalid input for bSize");

    // Initialize arrays a and b
    let mut a = vec![0; a_size];
    let mut b = vec![0; b_size];

    // Read the elements of array a
    input.clear();
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
    for (i, item) in input.trim().split_whitespace().enumerate() {
        a[i] = item.parse().expect("Invalid input for a");
    }

    // Read the elements of array b
    input.clear();
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
    for (i, item) in input.trim().split_whitespace().enumerate() {
        b[i] = item.parse().expect("Invalid input for b");
    }

    // Calculate and print the result
    let result = max_score(&a, &b);
    writeln!(stdout, "{}", result).expect("Failed to write to stdout");
}