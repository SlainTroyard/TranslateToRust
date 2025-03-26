use std::io::{self, BufRead};

fn cmp(a: &i32, b: &i32) -> std::cmp::Ordering {
    b.cmp(a)
}

fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &[i32], k: usize) -> i64 {
    let mut len = 0;
    for &limit in limits {
        len += limit;
    }

    let mut lst = vec![0; len as usize];
    let mut l = 0;

    for (i, row) in grid.iter_mut().enumerate() {
        row.sort_by(|a, b| cmp(a, b));
        for j in 0..limits[i] as usize {
            lst[l] = row[j];
            l += 1;
        }
    }

    lst.sort_by(|a, b| cmp(a, b));
    let ans: i64 = lst.iter().take(k).map(|&x| x as i64).sum();

    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n, m, k
    let first_line: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = first_line[0] as usize;
    let m = first_line[1] as usize;
    let k = first_line[2] as usize;

    // Read grid
    let mut grid = Vec::with_capacity(n);
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