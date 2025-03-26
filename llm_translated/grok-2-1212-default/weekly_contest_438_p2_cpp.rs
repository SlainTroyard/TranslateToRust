use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_sum(grid: &Vec<Vec<i32>>, limits: &Vec<i32>, k: i32) -> i64 {
        let n = grid.len();
        let m = grid[0].len();

        // Create a max heap
        let mut pq = BinaryHeap::new();
        for i in 0..n {
            let mut row = grid[i].clone();
            row.sort_by(|a, b| b.cmp(a));
            pq.push(Reverse((row[0], i as i32, 0)));
        }

        let mut ans: i64 = 0;
        let mut k = k;
        while k > 0 && !pq.is_empty() {
            let Reverse((val, r, c)) = pq.pop().unwrap();
            if c >= limits[r as usize] {
                continue;
            }
            ans += val as i64;
            k -= 1;
            if c + 1 < m as i32 {
                pq.push(Reverse((grid[r as usize][(c + 1) as usize], r, c + 1)));
            }
        }
        ans
    }
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
    let k = first_line[2];

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
    let result = Solution::max_sum(&grid, &limits, k);
    println!("{}", result);

    Ok(())
}