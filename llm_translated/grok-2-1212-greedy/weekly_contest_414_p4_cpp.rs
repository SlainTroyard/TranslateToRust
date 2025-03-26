use std::io::{self, BufRead};
use std::cmp::{max, min};

const DIRS: [(i32, i32); 8] = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];

struct Solution;

impl Solution {
    fn max_moves(&self, kx: i32, ky: i32, positions: &Vec<Vec<i32>>) -> i32 {
        let n = positions.len();
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];

        // Calculate the number of moves required for the knight to reach each position
        for i in 0..n {
            let (px, py) = (positions[i][0], positions[i][1]);
            dis[i][px as usize][py as usize] = 0;
            let mut q = vec![(px, py)];
            let mut step = 1;
            while !q.is_empty() {
                let mut tmp = Vec::new();
                for &(qx, qy) in &q {
                    for &(dx, dy) in &DIRS {
                        let x = qx + dx;
                        let y = qy + dy;
                        if x >= 0 && x < 50 && y >= 0 && y < 50 && dis[i][x as usize][y as usize] == -1 {
                            dis[i][x as usize][y as usize] = step;
                            tmp.push((x, y));
                        }
                    }
                }
                q = tmp;
                step += 1;
            }
        }

        // Add the knight's position to the list of positions
        let mut positions = positions.clone();
        positions.push(vec![kx, ky]);
        let mut memo = vec![vec![-1; 1 << n]; n + 1];
        let u = (1 << n) - 1;

        // Use a closure to allow recursion
        let dfs = |i: usize, mask: usize| -> i32 {
            if mask == 0 {
                return 0; // No more soldiers to move
            }
            if memo[i][mask] != -1 {
                return memo[i][mask]; // If the result has been calculated before, return it
            }
            let (x, y) = (positions[i][0], positions[i][1]);
            let res = if (u ^ mask).count_ones() % 2 == 0 { // Alice's move
                let mut res = i32::MIN;
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        res = max(res, dfs(j, mask ^ (1 << j)) + dis[j][x as usize][y as usize]);
                    }
                }
                res
            } else { // Bob's move
                let mut res = i32::MAX;
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        res = min(res, dfs(j, mask ^ (1 << j)) + dis[j][x as usize][y as usize]);
                    }
                }
                res
            };
            memo[i][mask] = res;
            res
        };

        dfs(n, u) // Start the DFS from the knight's position
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read knight's position (kx, ky) and number of soldiers
    let first_line: Vec<i32> = lines.next().unwrap()?.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (kx, ky, n) = (first_line[0], first_line[1], first_line[2] as usize);

    // Read the positions of soldiers
    let mut positions = Vec::new();
    for _ in 0..n {
        let line: Vec<i32> = lines.next().unwrap()?.split_whitespace().map(|s| s.parse().unwrap()).collect();
        positions.push(line);
    }

    // Call the maxMoves function and output the result
    let solution = Solution;
    let result = solution.max_moves(kx, ky, &positions);
    println!("{}", result);

    Ok(())
}