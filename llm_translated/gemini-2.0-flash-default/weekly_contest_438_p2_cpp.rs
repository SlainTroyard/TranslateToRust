use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

struct Solution {}

impl Solution {
    pub fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &Vec<i32>, k: i32) -> i64 {
        let n = grid.len();
        let m = grid[0].len();

        let mut pq: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        for i in 0..n {
            grid[i].sort_by_key(|&x| Reverse(x));
            pq.push((grid[i][0], i, 0));
        }

        let mut ans: i64 = 0;
        let mut k_remaining = k;
        while k_remaining > 0 && !pq.is_empty() {
            let (val, r, c) = pq.pop().unwrap();
            if c >= limits[r] as usize {
                continue;
            }
            ans += val as i64;
            k_remaining -= 1;
            if c + 1 < m {
                pq.push((grid[r][c + 1], r, c + 1));
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(row);
    }

    let mut limits: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let limit: i32 = input.trim().parse().unwrap();
        limits.push(limit);
    }

    let mut sol = Solution {};
    println!("{}", sol.max_sum(&mut grid, &limits, k));
}