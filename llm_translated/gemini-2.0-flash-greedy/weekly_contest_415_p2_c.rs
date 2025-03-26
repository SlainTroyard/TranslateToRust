use std::cmp::max;
use std::io;

fn max_score(a: &[i32], b: &[i32]) -> i64 {
    let n = b.len();
    let mut f = vec![vec![i64::MIN; 5]; n + 1];

    for j in 0..=4 {
        f[0][j] = i64::MIN; // Initialize all to min except f[0][0]
    }
    f[0][0] = 0;

    for i in 1..=n {
        for j in 0..=4 {
            f[i][j] = f[i - 1][j];
            if j > 0 {
                f[i][j] = max(f[i][j], f[i - 1][j - 1] + i64::from(a[j - 1]) * i64::from(b[i - 1]));
            }
        }
    }
    f[n][4]
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();

    let a_size: usize = iter.next().unwrap().parse().unwrap();
    let b_size: usize = iter.next().unwrap().parse().unwrap();

    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input)?;
    let a: Vec<i32> = a_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input)?;
    let b: Vec<i32> = b_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", max_score(&a, &b));

    Ok(())
}