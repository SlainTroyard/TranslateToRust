use std::cmp::{max, min};
use std::io::{self, Read};

/// A helper function to read all integers from stdin and return them in a vector.
fn read_ints() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    input.split_whitespace()
         .map(|s| s.parse().expect("Failed to parse integer"))
         .collect()
}

struct Solution;

impl Solution {
    /// Calculate the maximum number of moves following the described logic.
    /// kx, ky: Knight's position.
    /// positions: A mutable reference to the soldiers' positions.
    fn max_moves(&self, kx: i32, ky: i32, positions: &mut Vec<[i32; 2]>) -> i32 {
        let n = positions.len();

        // dis[i][x][y] will store the number of moves needed for soldier i to reach (x, y).
        // Initialize all distances to -1 (unvisited).
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];

        // Directions for the knight's movement (8 possible directions).
        static DIRS: &[(i32, i32)] = &[
            (2, 1), (1, 2), (-1, 2), (-2, 1),
            (-2, -1), (-1, -2), (1, -2), (2, -1),
        ];

        // Perform BFS for each soldier to find min steps to every cell on the 50x50 board.
        for i in 0..n {
            let (px, py) = (positions[i][0], positions[i][1]);
            // Mark the starting position of the soldier.
            dis[i][px as usize][py as usize] = 0;
            let mut q = vec![(px, py)];
            let mut step = 0;

            // Layer-by-layer BFS
            while !q.is_empty() {
                step += 1;
                let mut tmp = Vec::new();
                for (qx, qy) in q {
                    for &(dx, dy) in DIRS {
                        let nx = qx + dx;
                        let ny = qy + dy;
                        // Check board bounds and unvisited status.
                        if nx >= 0 && nx < 50 && ny >= 0 && ny < 50 {
                            if dis[i][nx as usize][ny as usize] == -1 {
                                dis[i][nx as usize][ny as usize] = step;
                                tmp.push((nx, ny));
                            }
                        }
                    }
                }
                q = tmp;
            }
        }

        // Append the knight's position to the end of the positions vector.
        positions.push([kx, ky]);

        // Create a memo table for DFS:
        // - (n+1) rows for soldier positions + knight's position
        // - (1 << n) columns for all subsets of soldiers
        let mut memo = vec![vec![-1; 1 << n]; n + 1];
        let u = (1 << n) - 1; // bitmask with n bits set

        /// A recursive DFS function with memoization.
        /// i: current index in positions (0..n for soldiers, n for knight).
        /// mask: bitmask representing which soldiers remain to be moved.
        /// positions: all positions (including knight's appended at index n).
        /// dis: distances for each soldier to all board cells.
        /// memo: memoization table to cache results.
        /// u: a mask with all soldier bits set (used for parity check).
        fn dfs(
            i: usize,
            mask: usize,
            positions: &[[i32; 2]],
            dis: &[Vec<Vec<i32>>],
            memo: &mut [Vec<i32>],
            u: usize,
        ) -> i32 {
            // If no soldiers are left to move, return 0.
            if mask == 0 {
                return 0;
            }
            // If we've already computed this state, return it.
            if memo[i][mask] != -1 {
                return memo[i][mask];
            }

            let (x, y) = (positions[i][0] as usize, positions[i][1] as usize);
            let parity = ((u ^ mask) as u32).count_ones() % 2;

            // Alice's move if parity == 0, else Bob's move.
            if parity == 0 {
                // Alice tries to maximize the result.
                let mut best = -1;
                for j in 0..dis.len() {
                    if (mask >> j) & 1 == 1 {
                        let val = dfs(j, mask ^ (1 << j), positions, dis, memo, u)
                                  + dis[j][x][y];
                        best = max(best, val);
                    }
                }
                memo[i][mask] = best;
            } else {
                // Bob tries to minimize the result.
                let mut best = i32::MAX;
                for j in 0..dis.len() {
                    if (mask >> j) & 1 == 1 {
                        let val = dfs(j, mask ^ (1 << j), positions, dis, memo, u)
                                  + dis[j][x][y];
                        best = min(best, val);
                    }
                }
                memo[i][mask] = best;
            }
            memo[i][mask]
        }

        // Start the DFS from the knight's position, with all soldiers available to move.
        dfs(n, u, positions, &dis, &mut memo, u)
    }
}

fn main() {
    // Read all input tokens at once.
    let tokens = read_ints();
    let mut idx = 0;

    // Read knight's position and number of soldiers.
    let kx = tokens[idx]; idx += 1;
    let ky = tokens[idx]; idx += 1;
    let n = tokens[idx] as usize; idx += 1;

    // Read the positions of the soldiers.
    let mut positions = Vec::new();
    for _ in 0..n {
        let x = tokens[idx];
        let y = tokens[idx + 1];
        idx += 2;
        positions.push([x, y]);
    }

    // Create a Solution instance and compute the result.
    let solution = Solution;
    let result = solution.max_moves(kx, ky, &mut positions);

    // Output the result.
    println!("{}", result);
}