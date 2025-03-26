use std::io;
use std::vec;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse()?;
    let m: usize = iter.next().unwrap().parse()?;
    let mut k: i32 = iter.next().unwrap().parse()?;

    let mut grid: Vec<Vec<i32>> = vec::Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    input.clear();
    io::stdin().read_line(&mut input)?;
    let limits: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let ans = max_sum(&mut grid, &limits, k);
    println!("{}", ans);

    Ok(())
}

fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &Vec<i32>, mut k: i32) -> i64 {
    let n = grid.len();
    let m = grid[0].len();

    let mut pq: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
    for i in 0..n {
        grid[i].sort_by_key(|&x| Reverse(x)); // Sort in descending order
        pq.push((grid[i][0], i, 0)); // (value, row_index, col_index)
    }

    let mut ans: i64 = 0;
    while k > 0 && !pq.is_empty() {
        if let Some((val, r, c)) = pq.pop() {
            if c >= limits[r] as usize {
                continue; // Skip if column index exceeds limit
            }
            ans += val as i64;
            k -= 1;
            if c + 1 < m {
                pq.push((grid[r][c + 1], r, c + 1)); // Push next element from the same row
            }
        }
    }
    ans
}

fn main() {
    if let Err(err) = solve() {
        eprintln!("Error: {}", err);
    }
}