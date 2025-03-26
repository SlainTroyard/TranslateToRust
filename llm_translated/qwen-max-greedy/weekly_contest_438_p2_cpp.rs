use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn max_sum(grid: &Vec<Vec<i32>>, limits: &Vec<i32>, k: i32) -> i64 {
        let n = grid.len();
        let m = grid[0].len();

        let mut pq = BinaryHeap::new();
        for (i, row) in grid.iter().enumerate() {
            let mut sorted_row = row.clone();
            sorted_row.sort_by(|a, b| b.cmp(a));
            pq.push(Reverse((sorted_row[0], i, 0)));
        }

        let mut ans: i64 = 0;
        while k > 0 && !pq.is_empty() {
            let Reverse((val, r, c)) = pq.pop().unwrap();
            if c >= limits[r] as usize {
                continue;
            }
            ans += val as i64;
            k -= 1;
            if c + 1 < m {
                pq.push(Reverse((grid[r][c + 1], r, c + 1)));
            }
        }
        ans
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = iter.next().unwrap().parse().expect("Failed to parse m");
    let k: i32 = iter.next().unwrap().parse().expect("Failed to parse k");

    // Initialize grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        for j in 0..m {
            grid[i][j] = iter.next().unwrap().parse().expect("Failed to parse grid value");
        }
    }

    // Initialize limits
    let mut limits = vec![0; n];
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    for i in 0..n {
        limits[i] = iter.next().unwrap().parse().expect("Failed to parse limit value");
    }

    // Solve and print result
    let sol = Solution;
    let result = sol.max_sum(&grid, &limits, k);
    println!("{}", result);
}