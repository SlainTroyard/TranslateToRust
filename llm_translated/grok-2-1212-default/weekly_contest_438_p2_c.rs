use std::io::{self, BufRead};

fn cmp(a: &i32, b: &i32) -> std::cmp::Ordering {
    b.cmp(a)
}

fn max_sum(grid: &mut [Vec<i32>], limits: &[i32], k: i32) -> i64 {
    let mut len = 0;
    for &limit in limits {
        len += limit;
    }

    let mut lst = vec![0; len as usize];
    let mut l = 0;

    for (i, row) in grid.iter_mut().enumerate() {
        row.sort_by(cmp);
        for j in 0..limits[i] as usize {
            lst[l] = row[j];
            l += 1;
        }
    }

    lst.sort_by(cmp);
    let mut ans: i64 = 0;
    for i in 0..k as usize {
        ans += lst[i] as i64;
    }

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n, m, k
    let first_line: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = first_line[0];
    let m = first_line[1];
    let k = first_line[2];

    // Read grid
    let mut grid = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let row: Vec<i32> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Read limits
    let limits: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print result
    let result = max_sum(&mut grid, &limits, k);
    println!("{}", result);

    Ok(())
}