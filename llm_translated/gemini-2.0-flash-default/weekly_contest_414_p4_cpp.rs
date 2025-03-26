use std::cmp::{max, min};
use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::FromStr;

// Helper function to read a single value from stdin
fn read_single_value<T: FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().ok().unwrap()
}

// Helper function to read a line of space-separated values from stdin
fn read_line_of_values<T: FromStr>() -> Vec<T> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
}

const DIRS: [[i32; 2]; 8] = [
    [2, 1],
    [1, 2],
    [-1, 2],
    [-2, 1],
    [-2, -1],
    [-1, -2],
    [1, -2],
    [2, -1],
];

struct Solution {}

impl Solution {
    fn max_moves(kx: i32, ky: i32, mut positions: Vec<Vec<i32>>) -> i32 {
        let n = positions.len();
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];

        // Calculate the number of moves required for the knight to reach each position
        for i in 0..n {
            let px = positions[i][0] as usize;
            let py = positions[i][1] as usize;
            dis[i][px][py] = 0;
            let mut q = VecDeque::from(vec![(px, py)]);

            // Perform a BFS to calculate the minimum steps from each soldier to all other positions
            let mut step = 1;
            while !q.is_empty() {
                let mut tmp = VecDeque::new();
                while let Some((qx, qy)) = q.pop_front() {
                    for &[dx, dy] in &DIRS {
                        let x = (qx as i32 + dx) as usize;
                        let y = (qy as i32 + dy) as usize;
                        // Ensure the new position is within bounds and not yet visited
                        if x < 50 && y < 50 && dis[i][x][y] < 0 {
                            dis[i][x][y] = step;
                            tmp.push_back((x, y));
                        }
                    }
                }
                q = tmp;
                step += 1;
            }
        }

        // Add the knight's position to the list of positions
        positions.push(vec![kx, ky]);
        let mut memo = vec![vec![-1; 1 << n]; n + 1];
        let u = (1 << n) - 1;

        // Use a closure to allow recursion
        let mut dfs = |i: usize, mask: i32| -> i32 {
            if mask == 0 {
                return 0; // No more soldiers to move
            }
            let res = &mut memo[i][mask as usize]; // Use reference to store the result
            if *res != -1 {
                // If the result has been calculated before, return it
                return *res;
            }

            let x = positions[i][0] as usize;
            let y = positions[i][1] as usize;

            if (u ^ mask).count_ones() % 2 == 0 {
                // Alice's move
                *res = i32::MIN;
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        *res = max(*res, dfs(j, mask ^ (1 << j)) + dis[j][x][y]);
                    }
                }
            } else {
                // Bob's move
                *res = i32::MAX;
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        *res = min(*res, dfs(j, mask ^ (1 << j)) + dis[j][x][y]);
                    }
                }
            }
            *res
        };

        dfs(n, u) // Start the DFS from the knight's position
    }
}

fn main() {
    let kx: i32 = read_single_value();
    let ky: i32 = read_single_value();
    let n: usize = read_single_value();

    let mut positions: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        positions.push(read_line_of_values());
    }

    let result = Solution {}.max_moves(kx, ky, positions);
    println!("{}", result);
}