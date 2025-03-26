use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n, m, k from the first line
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();
    let k = nums.next().unwrap();

    // Read the grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        for j in 0..m {
            grid[i][j] = nums.next().unwrap();
        }
    }

    // Read the limits
    let mut limits = vec![0; n];
    let line = lines.next().unwrap().unwrap();
    let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    for i in 0..n {
        limits[i] = nums.next().unwrap();
    }

    // Compute and print the result
    let result = max_sum(&mut grid, &limits, k);
    println!("{}", result);
}

fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &[usize], k: usize) -> i64 {
    let n = grid.len();
    let m = grid[0].len();

    // Sort each row in descending order
    for row in grid.iter_mut() {
        row.sort_by_key(|&x| Reverse(x));
    }

    // Use a max-heap to keep track of the largest elements
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((grid[i][0], i, 0));
    }

    let mut ans = 0i64;
    let mut remaining = k;

    while remaining > 0 && !heap.is_empty() {
        let (value, r, c) = heap.pop().unwrap();
        if c >= limits[r] {
            continue;
        }
        ans += value as i64;
        remaining -= 1;
        if c + 1 < m {
            heap.push((grid[r][c + 1], r, c + 1));
        }
    }

    ans
}