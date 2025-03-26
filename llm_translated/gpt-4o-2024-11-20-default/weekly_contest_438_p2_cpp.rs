use std::io;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<usize>, mut k: usize) -> i64 {
        let n = grid.len();
        let m = grid[0].len();

        // Priority queue to store elements in descending order
        let mut pq = BinaryHeap::new();

        for (i, mut row) in grid.into_iter().enumerate() {
            // Sort the row in descending order
            row.sort_unstable_by(|a, b| b.cmp(a));
            // Push the largest element along with its row index and column index
            pq.push((Reverse(row[0]), i, 0));
        }

        let mut ans = 0_i64;

        while k > 0 && !pq.is_empty() {
            let (Reverse(val), r, c) = pq.pop().unwrap();
            if c >= limits[r] {
                continue;
            }
            ans += val as i64;
            k -= 1;
            if c + 1 < m {
                // Push the next element in the same row into the priority queue
                pq.push((Reverse(grid[r][c + 1]), r, c + 1));
            }
        }

        ans
    }
}

fn main() {
    // Reading input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().expect("Invalid input for n");
    let m: usize = first_iter.next().unwrap().parse().expect("Invalid input for m");
    let k: usize = first_iter.next().unwrap().parse().expect("Invalid input for k");

    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap();
        grid[i] = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Invalid input for grid"))
            .collect();
    }

    let limits_line = lines.next().unwrap();
    let limits: Vec<usize> = limits_line
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Invalid input for limits"))
        .collect();

    let sol = Solution;
    let result = sol.max_sum(grid, limits, k);

    // Output the result
    println!("{}", result);
}