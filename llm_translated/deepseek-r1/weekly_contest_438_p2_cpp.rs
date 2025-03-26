use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = tokens.next().unwrap() as usize;
    let m = tokens.next().unwrap() as usize;
    let k = tokens.next().unwrap() as usize;

    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<i32> = (0..m).map(|_| tokens.next().unwrap()).collect();
        grid.push(row);
    }

    let mut limits = Vec::with_capacity(n);
    for _ in 0..n {
        limits.push(tokens.next().unwrap() as usize);
    }

    println!("{}", max_sum(&mut grid, &limits, k));
}

fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &[usize], k: usize) -> i64 {
    for row in grid.iter_mut() {
        row.sort_by(|a, b| b.cmp(a));
    }

    let mut heap = BinaryHeap::new();
    for (r, row) in grid.iter().enumerate() {
        heap.push((row[0], r, 0));
    }

    let mut ans = 0;
    let mut remaining = k;

    while remaining > 0 && !heap.is_empty() {
        let (val, r, c) = heap.pop().unwrap();

        if c >= limits[r] {
            continue;
        }

        ans += val as i64;
        remaining -= 1;

        if c + 1 < grid[r].len() {
            heap.push((grid[r][c + 1], r, c + 1));
        }
    }

    ans
}