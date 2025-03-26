use std::cmp::Reverse;
use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    pub fn max_sum(grid: &Vec<Vec<i32>>, limits: &Vec<i32>, k: i32) -> i64 {
        let n = grid.len();
        let m = grid[0].len();

        use std::collections::BinaryHeap;
        let mut pq: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        
        for (i, row) in grid.iter().enumerate() {
            let mut sorted_row = row.clone();
            sorted_row.sort_by(|a, b| b.cmp(a));
            pq.push((sorted_row[0], i, 0));
        }

        let mut ans: i64 = 0;
        while k > 0 && !pq.is_empty() {
            if let Some((val, r, c)) = pq.pop() {
                if c as i32 >= limits[r] {
                    continue;
                }
                ans += val as i64;
                if c + 1 < m {
                    pq.push((grid[r][c + 1], r, c + 1));
                }
            }
            k -= 1;
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    // Read input
    let mut input = String::new();
    stdin_lock.read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = iter.next().unwrap().parse().expect("Failed to parse m");
    let k: i32 = iter.next().unwrap().parse().expect("Failed to parse k");

    // Initialize grid
    let mut grid: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input).expect("Failed to read line");
        for (j, item) in input.trim().split_whitespace().enumerate() {
            grid[i][j] = item.parse().expect("Failed to parse grid value");
        }
    }

    // Initialize limits
    let mut limits: Vec<i32> = Vec::with_capacity(n);
    input.clear();
    stdin_lock.read_line(&mut input).expect("Failed to read line");
    for item in input.trim().split_whitespace() {
        limits.push(item.parse().expect("Failed to parse limit"));
    }

    // Solve the problem
    let sol = Solution;
    let result = sol.max_sum(&grid, &limits, k);

    // Output the result
    writeln!(stdout_lock, "{}", result).expect("Failed to write result");
}