use std::io::{self, Read};

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1];

    for j in 0..=4 {
        f[0][j] = 0;
    }

    for i in 1..=n {
        for j in 0..=4 {
            f[i][j] = f[i-1][j];
            if j > 0 {
                f[i][j] = f[i][j].max(f[i-1][j-1] + a[j-1] as i64 * b[i-1] as i64);
            }
        }
    }

    f[n][4]
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut numbers = first_line.split_whitespace();

    let a_size: usize = numbers.next().unwrap().parse().unwrap();
    let b_size: usize = numbers.next().unwrap().parse().unwrap();

    let a: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let b: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = max_score(&a, &b);
    println!("{}", result);

    Ok(())
}