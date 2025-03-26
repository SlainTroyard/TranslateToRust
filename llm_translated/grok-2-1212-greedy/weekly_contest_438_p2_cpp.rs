use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_sum(grid: &Vec<Vec<i32>>, limits: &Vec<i32>, k: i32) -> i64 {
        let n = grid.len();
        let m = grid[0].len();

        let mut pq = BinaryHeap::new();
        for i in 0..n {
            let mut row = grid[i].clone();
            row.sort_by(|a, b| b.cmp(a));
            pq.push(Reverse((row[0], i as i32, 0)));
        }

        let mut ans = 0;
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
    let first_line = lines.next().unwrap()?;
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();
    let k = nums.next().unwrap();

    // Read grid
    let mut grid = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        grid.push(row);
    }

    // Read limits
    let limits_line = lines.next().unwrap()?;
    let limits: Vec<i32> = limits_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Calculate and print result
    let result = Solution::max_sum(&grid, &limits, k);
    println!("{}", result);

    Ok(())
}