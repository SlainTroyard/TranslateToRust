use std::io::{self, BufRead, Write};

fn cmp(a: &i32, b: &i32) -> i32 {
    b.cmp(a).into()
}

fn max_sum(grid: &Vec<Vec<i32>>, limits: &Vec<usize>, k: usize) -> i64 {
    let mut lst: Vec<i32> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        let mut sorted_row = row.clone();
        sorted_row.sort_by_key(|&x| cmp(&x, &0));
        for j in 0..limits[i] {
            if let Some(&val) = sorted_row.get(j) {
                lst.push(val);
            }
        }
    }
    lst.sort_by_key(|&x| cmp(&x, &0));
    lst.into_iter().take(k).map(|x| x as i64).sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Read the first line to get n, m, and k
    stdin.lock().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        stdin.lock().read_line(&mut input)?;
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Initialize the limits
    let mut limits: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        stdin.lock().read_line(&mut input)?;
        let limit: usize = input.trim().parse().unwrap();
        limits.push(limit);
    }

    // Calculate the result
    let result = max_sum(&grid, &limits, k);

    // Output the result
    writeln!(stdout, "{}", result)?;

    Ok(())
}