use std::io;

fn maxScore(a: &[i32], aSize: usize, b: &[i32], bSize: usize) -> i64 {
    let n = bSize;
    let mut f = vec![vec![i64::MIN; 5]; n + 1];
    f[0][0] = 0;

    for i in 1..=n {
        for j in 0..=4 {
            f[i][j] = f[i-1][j];
            if j > 0 {
                let val = f[i-1][j-1] + (a[j-1] as i64) * (b[i-1] as i64);
                if val > f[i][j] {
                    f[i][j] = val;
                }
            }
        }
    }

    f[n][4]
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut parts = input.trim().split_whitespace();
    let aSize = parts.next().unwrap().parse::<usize>()?;
    let bSize = parts.next().unwrap().parse::<usize>()?;

    let mut a = vec![0; aSize];
    let mut b = vec![0; bSize];

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let a_parts: Vec<&str> = input.trim().split_whitespace().collect();
    for i in 0..aSize {
        a[i] = a_parts[i].parse::<i32>()?;
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let b_parts: Vec<&str> = input.trim().split_whitespace().collect();
    for i in 0..bSize {
        b[i] = b_parts[i].parse::<i32>()?;
    }

    let result = maxScore(&a, aSize, &b, bSize);
    println!("{}", result);
    Ok(())
}